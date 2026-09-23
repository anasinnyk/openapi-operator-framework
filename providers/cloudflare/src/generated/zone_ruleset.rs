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
            provider_core::CredentialError::MissingValue(
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
            provider_core::CredentialError::MissingValue(
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
