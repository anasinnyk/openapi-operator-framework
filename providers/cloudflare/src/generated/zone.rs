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
    pub account_ref: core::reference::ResourceReference,
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
    pub management: core::managed::ManagedResourceSpec,
}
