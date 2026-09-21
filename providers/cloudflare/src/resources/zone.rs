use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "cloudflare.cfrs.dev",
    version = "v1alpha1",
    kind = "Zone",
    namespaced,
    status = "ZoneStatus"
)]
pub struct ZoneSpec {
    pub name: String,
    #[serde(rename = "accountRef")]
    pub account_ref: AccountRef,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<ZoneType>,
    #[serde(rename = "jumpStart", skip_serializing_if = "Option::is_none")]
    pub jump_start: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ZoneStatus {
    pub id: String,
    #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
    pub name_servers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct AccountRef {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ZoneType {
    Full,
    Partial,
}
