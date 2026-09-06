use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use kube::{
    Api, Client, CustomResource, ResourceExt,
    runtime::{Controller, controller::Action, watcher},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "cloudflare.cfrs.dev",
    version = "v1alpha1",
    kind = "Account",
    namespaced
)]
pub struct AccountSpec {
    #[schemars(regex(pattern = r"^[a-z0-9]+$"))]
    pub id: String,
    #[serde(rename = "apiToken")]
    pub api_token: ApiTokenRef,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ApiTokenRef {
    #[serde(rename = "secretRef")]
    pub secret_ref: SecretKeyRef,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct SecretKeyRef {
    pub name: String,
    pub key: String,
}

#[derive(Clone)]
pub struct ControllerContext {
    client: Client,
}

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error(transparent)]
    Kube(#[from] kube::Error),
}

/// Run the Cloudflare Account controller until the watcher stream ends.
///
/// # Errors
///
/// Returns an error if controller initialization fails before the watch loop starts.
pub async fn run_controller(client: Client) -> Result<(), ControllerError> {
    let accounts = Api::<Account>::all(client.clone());
    let context = Arc::new(ControllerContext { client });

    Controller::new(accounts, watcher::Config::default())
        .run(reconcile, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok((account, action)) => {
                    tracing::info!(
                        account = %account.name,
                        ?action,
                        "reconciled Cloudflare Account"
                    );
                }
                Err(error) => {
                    tracing::error!(?error, "Cloudflare Account reconciliation failed");
                }
            }
        })
        .await;

    Ok(())
}

async fn reconcile(
    account: Arc<Account>,
    context: Arc<ControllerContext>,
) -> Result<Action, ControllerError> {
    let _client = context.client.clone();

    tracing::info!(account = %account.name_any(), "noop reconciliation");

    Ok(Action::requeue(Duration::from_mins(5)))
}

#[allow(clippy::needless_pass_by_value)]
fn error_policy(
    account: Arc<Account>,
    error: &ControllerError,
    _context: Arc<ControllerContext>,
) -> Action {
    tracing::warn!(
        account = %account.name_any(),
        ?error,
        "scheduling retry after reconciliation error"
    );

    Action::requeue(Duration::from_mins(1))
}
