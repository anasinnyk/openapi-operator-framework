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
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
}
