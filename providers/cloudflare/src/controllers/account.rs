use std::{sync::Arc, time::Duration};

use kube::{ResourceExt, runtime::controller::Action};

use crate::{
    controllers::{ControllerContext, ControllerError},
    resources::Account,
};

pub(super) async fn reconcile(
    account: Arc<Account>,
    context: Arc<ControllerContext>,
) -> Result<Action, ControllerError> {
    let _client = context.client.clone();

    tracing::info!(account = %account.name_any(), "noop reconciliation");

    Ok(Action::requeue(Duration::from_mins(5)))
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn error_policy(
    account: Arc<Account>,
    error: &ControllerError,
    _context: Arc<ControllerContext>,
) -> Action {
    tracing::warn!(
        account = %account.name_any(),
        ?error,
        "scheduling Account retry after reconciliation error"
    );

    Action::requeue(Duration::from_mins(1))
}
