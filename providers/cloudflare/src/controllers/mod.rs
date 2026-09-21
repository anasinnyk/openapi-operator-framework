use std::sync::Arc;

use futures::StreamExt;
use kube::{
    Api, Client,
    runtime::{Controller, watcher},
};
use thiserror::Error;

use crate::{
    cloudflare::{CloudflareClientFactory, CloudflareError},
    resources::{Account, DnsRecord, PagesProject, Zone, ZoneRuleset},
};

mod account;
mod generated;

#[derive(Clone)]
pub(super) struct ControllerContext {
    pub(super) client: Client,
    pub(super) cloudflare: CloudflareClientFactory,
}

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error(transparent)]
    Kube(#[from] kube::Error),

    #[error(transparent)]
    Cloudflare(#[from] CloudflareError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("resource {kind}/{name} must be namespaced")]
    MissingNamespace { kind: &'static str, name: String },

    #[error("resource {kind}/{name} does not have status.id yet")]
    MissingStatusId { kind: &'static str, name: String },

    #[error("referenced Secret/{name} does not contain key {key}")]
    MissingSecretKey { name: String, key: String },

    #[error("Secret/{name} key {key} is not valid UTF-8")]
    SecretValueUtf8 {
        name: String,
        key: String,
        #[source]
        error: std::string::FromUtf8Error,
    },
}

/// Run the Cloudflare controllers until the watcher streams end.
///
/// # Errors
///
/// Returns an error if controller initialization fails before the watch loop starts.
pub async fn run_controller(client: Client) -> Result<(), ControllerError> {
    let accounts = Api::<Account>::all(client.clone());
    let zones = Api::<Zone>::all(client.clone());
    let dns_records = Api::<DnsRecord>::all(client.clone());
    let zone_rulesets = Api::<ZoneRuleset>::all(client.clone());
    let pages_projects = Api::<PagesProject>::all(client.clone());
    let context = Arc::new(ControllerContext {
        client,
        cloudflare: CloudflareClientFactory::default(),
    });

    let account_controller = Controller::new(accounts, watcher::Config::default())
        .run(account::reconcile, account::error_policy, context.clone())
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
        });

    let zone_controller = Controller::new(zones, watcher::Config::default())
        .run(
            generated::zone::reconcile,
            generated::zone::error_policy,
            context.clone(),
        )
        .for_each(|result| async move {
            match result {
                Ok((zone, action)) => {
                    tracing::info!(
                        zone = %zone.name,
                        ?action,
                        "reconciled Cloudflare Zone"
                    );
                }
                Err(error) => {
                    tracing::error!(?error, "Cloudflare Zone reconciliation failed");
                }
            }
        });

    let dns_record_controller = Controller::new(dns_records, watcher::Config::default())
        .run(
            generated::dns_record::reconcile,
            generated::dns_record::error_policy,
            context.clone(),
        )
        .for_each(|result| async move {
            match result {
                Ok((dns_record, action)) => {
                    tracing::info!(
                        dns_record = %dns_record.name,
                        ?action,
                        "reconciled Cloudflare DnsRecord"
                    );
                }
                Err(error) => {
                    tracing::error!(?error, "Cloudflare DnsRecord reconciliation failed");
                }
            }
        });

    let zone_ruleset_controller = Controller::new(zone_rulesets, watcher::Config::default())
        .run(
            generated::zone_ruleset::reconcile,
            generated::zone_ruleset::error_policy,
            context.clone(),
        )
        .for_each(|result| async move {
            match result {
                Ok((zone_ruleset, action)) => {
                    tracing::info!(
                        zone_ruleset = %zone_ruleset.name,
                        ?action,
                        "reconciled Cloudflare ZoneRuleset"
                    );
                }
                Err(error) => {
                    tracing::error!(?error, "Cloudflare ZoneRuleset reconciliation failed");
                }
            }
        });

    let pages_project_controller = Controller::new(pages_projects, watcher::Config::default())
        .run(
            generated::pages_project::reconcile,
            generated::pages_project::error_policy,
            context,
        )
        .for_each(|result| async move {
            match result {
                Ok((pages_project, action)) => {
                    tracing::info!(
                        pages_project = %pages_project.name,
                        ?action,
                        "reconciled Cloudflare PagesProject"
                    );
                }
                Err(error) => {
                    tracing::error!(?error, "Cloudflare PagesProject reconciliation failed");
                }
            }
        });

    tokio::join!(
        account_controller,
        zone_controller,
        dns_record_controller,
        zone_ruleset_controller,
        pages_project_controller,
    );

    Ok(())
}
