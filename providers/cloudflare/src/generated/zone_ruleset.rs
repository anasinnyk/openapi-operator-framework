#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetAtProvider {
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
pub struct RulesetStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    pub at_provider: Option<RulesetAtProvider>,
    #[serde(
        rename = "observedGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_generation: Option<i64>,
    #[serde(default)]
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetForProvider {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "zoneRef")]
    pub zone_ref: core::reference::ResourceReference,
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
    group = "rulesets.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "Ruleset",
    namespaced,
    status = "RulesetStatus"
)]
pub struct RulesetSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: RulesetForProvider,
    #[serde(flatten)]
    pub management: core::managed::ManagedResourceSpec,
}
pub async fn resolve_credentials(
    client: &kube::Client,
    resource: &Ruleset,
) -> Result<crate::generated::client::ApiTokenCredential, core::api::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            core::api::CredentialError::MissingValue(
                format!("{} has no namespace", "Ruleset",),
            )
        })?;
    let reference_0 = (resource.spec.for_provider.zone_ref)
        .as_ref()
        .ok_or_else(|| {
            core::error::CredentialError::MissingValue(
                format!("resource reference {} is not set", "spec.forProvider.zoneRef",),
            )
        })?;
    let namespace_0 = reference_0.namespace.clone().unwrap_or_else(|| namespace.clone());
    let api_0: kube::Api<crate::generated::zone::Zone> = kube::Api::namespaced(
        client.clone(),
        &namespace_0,
    );
    let related_0 = api_0.get(&reference_0.name).await?;
    let reference_1 = (related_0.spec.for_provider.account_ref)
        .as_ref()
        .ok_or_else(|| {
            core::error::CredentialError::MissingValue(
                format!(
                    "resource reference {} is not set", "spec.forProvider.accountRef",
                ),
            )
        })?;
    let namespace_1 = reference_1
        .namespace
        .clone()
        .unwrap_or_else(|| namespace_0.clone());
    let api_1: kube::Api<crate::generated::account::Account> = kube::Api::namespaced(
        client.clone(),
        &namespace_1,
    );
    let related_1 = api_1.get(&reference_1.name).await?;
    let selector = (related_1.spec.for_provider.api_token_secret_ref)
        .as_ref()
        .ok_or_else(|| {
            core::api::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = core::reference::resolve_secret_key(client, &namespace_1, selector)
        .await?;
    Ok(crate::generated::client::ApiTokenCredential::new(value)?)
}
pub async fn observe(
    kube_client: &kube::Client,
    provider_client: &crate::generated::client::ProviderClient,
    resource: &Ruleset,
) -> Result<core::reconciler::Observation, core::error::ReconcileError> {
    let ruleset_id = (async {
        core::api::resolve_field_value(&resource, "status.atProvider.id")
    })
        .await?
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "ruleset_id",),
            )
        })?;
    let zone_id = (async {
        let root_namespace = kube::ResourceExt::namespace(resource)
            .ok_or_else(|| {
                core::error::ResolveValueError::Missing(
                    "resource has no namespace".into(),
                )
            })?;
        let reference_0 = (resource.spec.for_provider.zone_ref)
            .as_ref()
            .ok_or_else(|| {
                core::error::ResolveValueError::Missing(
                    format!(
                        "resource reference {} is not set", "spec.forProvider.zoneRef",
                    ),
                )
            })?;
        let namespace_0 = reference_0
            .namespace
            .clone()
            .unwrap_or_else(|| root_namespace.clone());
        let api_0: kube::Api<crate::generated::zone::Zone> = kube::Api::namespaced(
            client.clone(),
            &namespace_0,
        );
        let related_0 = api_0.get(&reference_0.name).await?;
        (async { core::api::resolve_field_value(&&related_0, "status.atProvider.id") })
            .await
    })
        .await?
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "zone_id",),
            )
        })?;
    let request = provider_client.get_zone_ruleset(&ruleset_id, &zone_id);
    let credentials = resolve_credentials(kube_client, resource).await?;
    let request = request.with_credentials(&credentials);
    let observed = request.send_optional().await?;
    let at_provider = observed.map(serde_json::to_value).transpose()?;
    Ok(core::reconciler::Observation {
        exists: at_provider.is_some(),
        at_provider,
    })
}
