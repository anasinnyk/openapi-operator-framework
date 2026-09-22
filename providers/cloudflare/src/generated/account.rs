#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct AccountAtProvider {
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
pub struct AccountStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    pub at_provider: Option<AccountAtProvider>,
    #[serde(
        rename = "observedGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_generation: Option<i64>,
    #[serde(default)]
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}
pub type IamCommonComponentsSchemasIdentifier = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct AccountForProviderManagedBy {
    ///ID of the parent Organization, if one exists
    #[serde(rename = "parent_org_id", default, skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<String>,
    ///Name of the parent Organization, if one exists
    #[serde(
        rename = "parent_org_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_org_name: Option<String>,
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
pub struct AccountForProviderSettings {
    ///Sets an abuse contact email to notify for abuse reports.
    #[serde(
        rename = "abuse_contact_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub abuse_contact_email: Option<String>,
    /**Indicates whether membership in this account requires that
Two-Factor Authentication is enabled*/
    #[serde(
        rename = "enforce_twofactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enforce_twofactor: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type IamAccountType = serde_json::Value;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct AccountForProvider {
    #[serde(rename = "id")]
    pub id: IamCommonComponentsSchemasIdentifier,
    ///Parent container details
    #[serde(rename = "managed_by", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<AccountForProviderManagedBy>,
    ///Account name
    #[serde(rename = "name")]
    pub name: String,
    ///Account settings
    #[serde(rename = "settings", default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountForProviderSettings>,
    #[serde(rename = "type")]
    pub r#type: IamAccountType,
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
    kind = "Account",
    namespaced,
    status = "AccountStatus"
)]
pub struct AccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccountForProvider,
    #[serde(flatten)]
    pub management: provider_core::managed::ManagedResourceSpec,
}
