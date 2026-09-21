use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "cloudflare.cfrs.dev",
    version = "v1alpha1",
    kind = "PagesProject",
    namespaced,
    status = "PagesProjectStatus"
)]
pub struct PagesProjectSpec {
    #[serde(rename = "accountRef")]
    pub account_ref: PagesAccountRef,
    pub name: String,
    #[serde(rename = "productionBranch", skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<String>,
    #[serde(rename = "buildConfig", skip_serializing_if = "Option::is_none")]
    pub build_config: Option<serde_json::Value>,
    #[serde(rename = "deploymentConfigs", skip_serializing_if = "Option::is_none")]
    pub deployment_configs: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct PagesProjectStatus {
    pub id: String,
    #[serde(rename = "subdomain", skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct PagesAccountRef {
    pub name: String,
}
