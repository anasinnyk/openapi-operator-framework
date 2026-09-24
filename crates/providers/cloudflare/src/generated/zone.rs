//! This file is generated. Do not edit manually.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderAccount {
    ///Identifier
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The name of the account.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderMeta {
    ///The zone is only configured for CDN.
    #[serde(rename = "cdn_only", default, skip_serializing_if = "Option::is_none")]
    pub cdn_only: Option<bool>,
    ///Number of Custom Certificates the zone can have.
    #[serde(
        rename = "custom_certificate_quota",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_certificate_quota: Option<i64>,
    ///The zone is only configured for DNS.
    #[serde(rename = "dns_only", default, skip_serializing_if = "Option::is_none")]
    pub dns_only: Option<bool>,
    ///The zone is setup with Foundation DNS.
    #[serde(rename = "foundation_dns", default, skip_serializing_if = "Option::is_none")]
    pub foundation_dns: Option<bool>,
    ///Number of Page Rules a zone can have.
    #[serde(
        rename = "page_rule_quota",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_rule_quota: Option<i64>,
    ///The zone has been flagged for phishing.
    #[serde(
        rename = "phishing_detected",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phishing_detected: Option<bool>,
    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesIdentifier = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderOwner {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    ///Name of the owner.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The type of owner.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesPaused = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderPlan {
    ///States if the subscription can be activated.
    #[serde(rename = "can_subscribe", default, skip_serializing_if = "Option::is_none")]
    pub can_subscribe: Option<bool>,
    ///The denomination of the customer.
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///If this Zone is managed by another company.
    #[serde(
        rename = "externally_managed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub externally_managed: Option<bool>,
    ///How often the customer is billed.
    #[serde(rename = "frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    ///States if the subscription active.
    #[serde(rename = "is_subscribed", default, skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    ///If the legacy discount applies to this Zone.
    #[serde(
        rename = "legacy_discount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_discount: Option<bool>,
    ///The legacy name of the plan.
    #[serde(rename = "legacy_id", default, skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,
    ///Name of the owner.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///How much the customer is paying.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderTenant {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    ///The name of the Tenant account.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProviderTenantUnit {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesType = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneForProvider {
    ///The account the zone belongs to.
    #[serde(rename = "account")]
    pub account: ZoneForProviderAccount,
    /**Allows the customer to use a custom apex.
*Tenants Only Configuration*.*/
    #[serde(rename = "cname_suffix", default, skip_serializing_if = "Option::is_none")]
    pub cname_suffix: Option<String>,
    ///Identifier
    #[serde(rename = "id")]
    pub id: String,
    ///Metadata about the zone.
    #[serde(rename = "meta")]
    pub meta: ZoneForProviderMeta,
    ///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
    #[serde(rename = "name")]
    pub name: String,
    ///The owner of the zone.
    #[serde(rename = "owner")]
    pub owner: ZoneForProviderOwner,
    #[serde(rename = "paused", default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<ZonesPaused>,
    ///Legacy permissions based on legacy user membership information.
    #[serde(rename = "permissions", default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    ///A Zones subscription information.
    #[serde(rename = "plan")]
    pub plan: ZoneForProviderPlan,
    ///The root organizational unit that this zone belongs to (such as a tenant or organization).
    #[serde(rename = "tenant", default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<ZoneForProviderTenant>,
    ///The immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization).
    #[serde(rename = "tenant_unit", default, skip_serializing_if = "Option::is_none")]
    pub tenant_unit: Option<ZoneForProviderTenantUnit>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ZonesType>,
    ///An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    #[serde(
        rename = "vanity_name_servers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanity_name_servers: Option<Vec<String>>,
    #[serde(rename = "accountRef")]
    pub account_ref: Option<koof::reference::ResourceReference>,
}
#[derive(
    kube::CustomResource,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
#[kube(
    group = "cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "Zone",
    namespaced,
    status = "ZoneStatus"
)]
pub struct ZoneSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ZoneForProvider,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneAtProvider {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZoneStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    pub at_provider: Option<ZoneAtProvider>,
    #[serde(
        rename = "observedGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_generation: Option<i64>,
    #[serde(default)]
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}
pub async fn observe(
    kube_client: &kube::Client,
    provider_client: &crate::generated::client::ProviderClient,
    resource: &Zone,
) -> Result<koof::reconciler::Observation, koof::error::ReconcileError> {
    let zone_id = (async {
        koof::reference::resolve_field_value(resource, "status.atProvider.id")
    })
        .await?
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "zone_id",),
            )
        })?;
    let request = provider_client.zones_0_get(&zone_id);
    let credentials = resolve_credentials(kube_client, resource).await?;
    let request = request.with_credentials(&credentials);
    let observed = request.send_optional().await?;
    let at_provider = observed.map(serde_json::to_value).transpose()?;
    Ok(koof::reconciler::Observation {
        exists: at_provider.is_some(),
        at_provider,
    })
}
pub async fn resolve_credentials(
    client: &kube::Client,
    resource: &Zone,
) -> Result<crate::generated::client::ApiTokenCredential, koof::error::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!("{} has no namespace", "Zone",),
            )
        })?;
    let reference_0 = (resource.spec.for_provider.account_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!(
                    "resource reference {} is not set", "spec.forProvider.accountRef",
                ),
            )
        })?;
    let namespace_0 = reference_0.namespace.clone().unwrap_or_else(|| namespace.clone());
    let api_0: kube::Api<crate::generated::account::Account> = kube::Api::namespaced(
        client.clone(),
        &namespace_0,
    );
    let related_0 = api_0.get(&reference_0.name).await?;
    let selector = (related_0.spec.for_provider.api_token_secret_ref)
        .as_ref()
        .ok_or_else(|| {
            koof::error::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = koof::reference::resolve_secret_key(client, &namespace_0, selector)
        .await?;
    Ok(crate::generated::client::ApiTokenCredential::new(value)?)
}
pub async fn update_status(
    client: &kube::Client,
    resource: &Zone,
    observation: &koof::reconciler::Observation,
) -> Result<(), koof::error::ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            koof::error::ResolveValueError::Missing("resource has no namespace".into())
        })?;
    let name = kube::ResourceExt::name_any(resource);
    let at_provider: Option<ZoneAtProvider> = observation
        .at_provider
        .clone()
        .map(serde_json::from_value)
        .transpose()?;
    let condition_status = if observation.exists { "True" } else { "False" };
    let reason = if observation.exists { "Available" } else { "NotFound" };
    let message = if observation.exists {
        "External resource exists"
    } else {
        "External resource does not exist"
    };
    let previous_condition = resource
        .status
        .as_ref()
        .and_then(|status| {
            status.conditions.iter().find(|condition| { condition.type_ == "Ready" })
        });
    let last_transition_time = previous_condition
        .filter(|condition| {
            condition.status == condition_status && condition.reason == reason
        })
        .map(|condition| { condition.last_transition_time.clone() })
        .unwrap_or_else(|| {
            k8s_openapi::apimachinery::pkg::apis::meta::v1::Time::from(
                jiff::Timestamp::now(),
            )
        });
    let condition = k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition {
        type_: "Ready".into(),
        status: condition_status.into(),
        reason: reason.into(),
        message: message.into(),
        observed_generation: resource.metadata.generation,
        last_transition_time,
    };
    let status = ZoneStatus {
        at_provider,
        observed_generation: resource.metadata.generation,
        conditions: vec![condition],
    };
    let patch = serde_json::json!({ "status" : status, });
    let api: kube::Api<Zone> = kube::Api::namespaced(client.clone(), &namespace);
    let current_status = serde_json::to_value(&resource.status)?;
    let desired_status = serde_json::to_value(Some(&status))?;
    if current_status == desired_status {
        return Ok(());
    }
    api.patch_status(
            &name,
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(&patch),
        )
        .await?;
    Ok(())
}
pub async fn reconcile(
    resource: std::sync::Arc<Zone>,
    context: std::sync::Arc<
        koof::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) -> Result<kube::runtime::controller::Action, koof::error::ReconcileError> {
    let observation = observe(
            &context.kube_client,
            &context.provider_client,
            resource.as_ref(),
        )
        .await?;
    update_status(&context.kube_client, resource.as_ref(), &observation).await?;
    let requeue_after = if observation.exists {
        std::time::Duration::from_secs(300)
    } else {
        std::time::Duration::from_secs(30)
    };
    Ok(kube::runtime::controller::Action::requeue(requeue_after))
}
pub fn error_policy(
    _resource: std::sync::Arc<Zone>,
    _error: &koof::error::ReconcileError,
    _context: std::sync::Arc<
        koof::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) -> kube::runtime::controller::Action {
    kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(30))
}
pub async fn run_controller(
    context: std::sync::Arc<
        koof::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) {
    use futures::StreamExt as _;
    let api: kube::Api<Zone> = kube::Api::all(context.kube_client.clone());
    kube::runtime::Controller::new(api, kube::runtime::watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok((object, action)) => {
                    tracing::debug!(
                        resource = "Zone", ? object, ? action,
                        "reconciliation completed",
                    );
                }
                Err(error) => {
                    tracing::error!(
                        resource = "Zone", ? error, "reconciliation failed",
                    );
                }
            }
        })
        .await;
}
