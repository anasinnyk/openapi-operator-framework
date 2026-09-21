use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
