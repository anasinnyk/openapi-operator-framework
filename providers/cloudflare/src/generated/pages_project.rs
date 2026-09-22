#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectStatus {
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
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesBuildConfig {
    ///Enable build caching for the project.
    #[serde(rename = "build_caching", default, skip_serializing_if = "Option::is_none")]
    pub build_caching: Option<bool>,
    ///Command used to build project.
    #[serde(rename = "build_command", default, skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    ///Assets output directory of the build.
    #[serde(
        rename = "destination_dir",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_dir: Option<String>,
    ///Directory to run the command.
    #[serde(rename = "root_dir", default, skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,
    ///The classifying tag for analytics.
    #[serde(
        rename = "web_analytics_tag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics_tag: Option<String>,
    ///The auth token for analytics.
    #[serde(
        rename = "web_analytics_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics_token: Option<String>,
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
pub struct PagesProjectSpecCanonicalDeploymentDeploymentTriggerMetadata {
    ///Where the trigger happened.
    #[serde(rename = "branch")]
    pub branch: String,
    ///Whether the deployment trigger commit was dirty.
    #[serde(rename = "commit_dirty")]
    pub commit_dirty: bool,
    ///Hash of the deployment trigger commit.
    #[serde(rename = "commit_hash")]
    pub commit_hash: String,
    ///Message of the deployment trigger commit.
    #[serde(rename = "commit_message")]
    pub commit_message: String,
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
pub struct PagesProjectSpecCanonicalDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectSpecCanonicalDeploymentDeploymentTriggerMetadata,
    ///What caused the deployment.
    #[serde(rename = "type")]
    pub r#type: String,
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
pub struct PagesPlainTextEnvVar {
    #[serde(rename = "type")]
    pub r#type: String,
    ///Environment variable value.
    #[serde(rename = "value")]
    pub value: String,
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
pub struct PagesSecretTextEnvVar {
    #[serde(rename = "type")]
    pub r#type: String,
    ///Secret value.
    #[serde(rename = "value")]
    pub value: String,
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
#[serde(untagged)]
pub enum PagesEnvVarsAdditionalProperty {
    Variant1(PagesPlainTextEnvVar),
    Variant2(PagesSecretTextEnvVar),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesEnvVars {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesEnvVarsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesStage {
    ///When the stage ended.
    #[serde(rename = "ended_on", default, skip_serializing_if = "Option::is_none")]
    pub ended_on: Option<String>,
    ///The current build stage.
    #[serde(rename = "name")]
    pub name: String,
    ///When the stage started.
    #[serde(rename = "started_on", default, skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    ///State of the current stage.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type PagesProjectName = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    #[serde(rename = "deployments_enabled")]
    pub deployments_enabled: bool,
    ///The owner of the repository.
    #[serde(rename = "owner")]
    pub owner: String,
    ///The owner ID of the repository.
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_excludes")]
    pub path_excludes: Vec<String>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_includes")]
    pub path_includes: Vec<String>,
    ///Whether to enable PR comments.
    #[serde(rename = "pr_comments_enabled")]
    pub pr_comments_enabled: bool,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(rename = "preview_branch_excludes")]
    pub preview_branch_excludes: Vec<String>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(rename = "preview_branch_includes")]
    pub preview_branch_includes: Vec<String>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    #[serde(rename = "preview_deployment_setting")]
    pub preview_deployment_setting: String,
    ///The production branch of the repository.
    #[serde(rename = "production_branch")]
    pub production_branch: String,
    ///Whether to trigger a production deployment on commits to the production branch.
    #[serde(rename = "production_deployments_enabled")]
    pub production_deployments_enabled: bool,
    ///The ID of the repository.
    #[serde(rename = "repo_id")]
    pub repo_id: String,
    ///The name of the repository.
    #[serde(rename = "repo_name")]
    pub repo_name: String,
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
pub struct PagesSource {
    #[serde(rename = "config")]
    pub config: PagesSourceConfig,
    ///The source control management provider.
    #[serde(rename = "type")]
    pub r#type: String,
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
pub struct PagesProjectSpecCanonicalDeployment {
    ///A list of alias URLs pointing to this deployment.
    #[serde(rename = "aliases", default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "build_config")]
    pub build_config: PagesBuildConfig,
    ///When the deployment was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    ///Info about what caused the deployment.
    #[serde(rename = "deployment_trigger")]
    pub deployment_trigger: PagesProjectSpecCanonicalDeploymentDeploymentTrigger,
    #[serde(rename = "env_vars")]
    pub env_vars: PagesEnvVars,
    ///Type of deploy.
    #[serde(rename = "environment")]
    pub environment: String,
    ///Id of the deployment.
    #[serde(rename = "id")]
    pub id: String,
    ///If the deployment has been skipped.
    #[serde(rename = "is_skipped")]
    pub is_skipped: bool,
    #[serde(rename = "latest_stage")]
    pub latest_stage: PagesStage,
    ///When the deployment was last modified.
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    ///Id of the project.
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_name")]
    pub project_name: PagesProjectName,
    ///Short Id (8 character) of the deployment.
    #[serde(rename = "short_id")]
    pub short_id: String,
    ///Why the deployment was skipped.
    #[serde(rename = "skip_reason", default, skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,
    #[serde(rename = "source")]
    pub source: PagesSource,
    ///List of past stages.
    #[serde(rename = "stages")]
    pub stages: Vec<PagesStage>,
    ///The live URL to view this deployment.
    #[serde(rename = "url")]
    pub url: String,
    ///Whether the deployment uses functions.
    #[serde(rename = "uses_functions", default, skip_serializing_if = "Option::is_none")]
    pub uses_functions: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewAiBindingsAdditionalProperty {
    #[serde(rename = "project_id")]
    pub project_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewAiBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty {
    ///Name of the dataset.
    #[serde(rename = "dataset")]
    pub dataset: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewBrowsersAdditionalProperty {
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
pub struct PagesProjectSpecDeploymentConfigsPreviewBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewBrowsersAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewD1DatabasesAdditionalProperty {
    ///UUID of the D1 database.
    #[serde(rename = "id")]
    pub id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewD1DatabasesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty {
    ///ID of the Durable Object namespace.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty {
    #[serde(rename = "id")]
    pub id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewKvNamespacesAdditionalProperty {
    ///ID of the KV namespace.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewKvNamespacesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewLimits {
    ///CPU time limit in milliseconds.
    #[serde(rename = "cpu_ms")]
    pub cpu_ms: i64,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty {
    #[serde(rename = "certificate_id")]
    pub certificate_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewPlacement {
    ///Placement mode.
    #[serde(rename = "mode")]
    pub mode: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewQueueProducersAdditionalProperty {
    ///Name of the Queue.
    #[serde(rename = "name")]
    pub name: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewQueueProducersAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewR2BucketsAdditionalProperty {
    ///Jurisdiction of the R2 bucket.
    #[serde(rename = "jurisdiction", default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    ///Name of the R2 bucket.
    #[serde(rename = "name")]
    pub name: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewR2BucketsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment")]
    pub environment: String,
    ///The Service name.
    #[serde(rename = "service")]
    pub service: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewServicesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty {
    #[serde(rename = "index_name")]
    pub index_name: String,
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
pub struct PagesProjectSpecDeploymentConfigsPreviewVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsPreview {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<PagesProjectSpecDeploymentConfigsPreviewAiBindings>,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(rename = "always_use_latest_compatibility_date")]
    pub always_use_latest_compatibility_date: bool,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectSpecDeploymentConfigsPreviewAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectSpecDeploymentConfigsPreviewBrowsers>,
    ///The major version of the build image to use for Pages Functions.
    #[serde(rename = "build_image_major_version")]
    pub build_image_major_version: i64,
    ///Compatibility date used for Pages Functions.
    #[serde(rename = "compatibility_date")]
    pub compatibility_date: String,
    ///Compatibility flags used for Pages Functions.
    #[serde(rename = "compatibility_flags")]
    pub compatibility_flags: Vec<String>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<PagesProjectSpecDeploymentConfigsPreviewD1Databases>,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectSpecDeploymentConfigsPreviewDurableObjectNamespaces,
    >,
    #[serde(rename = "env_vars")]
    pub env_vars: PagesEnvVars,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open")]
    pub fail_open: bool,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectSpecDeploymentConfigsPreviewHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<PagesProjectSpecDeploymentConfigsPreviewKvNamespaces>,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectSpecDeploymentConfigsPreviewLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectSpecDeploymentConfigsPreviewMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectSpecDeploymentConfigsPreviewPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<PagesProjectSpecDeploymentConfigsPreviewQueueProducers>,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectSpecDeploymentConfigsPreviewR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectSpecDeploymentConfigsPreviewServices>,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model")]
    pub usage_model: String,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectSpecDeploymentConfigsPreviewVectorizeBindings,
    >,
    ///Hash of the Wrangler configuration used for the deployment.
    #[serde(
        rename = "wrangler_config_hash",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wrangler_config_hash: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionAiBindingsAdditionalProperty {
    #[serde(rename = "project_id")]
    pub project_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionAiBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty {
    ///Name of the dataset.
    #[serde(rename = "dataset")]
    pub dataset: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionBrowsersAdditionalProperty {
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
pub struct PagesProjectSpecDeploymentConfigsProductionBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionBrowsersAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionD1DatabasesAdditionalProperty {
    ///UUID of the D1 database.
    #[serde(rename = "id")]
    pub id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionD1DatabasesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty {
    ///ID of the Durable Object namespace.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty {
    #[serde(rename = "id")]
    pub id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionKvNamespacesAdditionalProperty {
    ///ID of the KV namespace.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionKvNamespacesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionLimits {
    ///CPU time limit in milliseconds.
    #[serde(rename = "cpu_ms")]
    pub cpu_ms: i64,
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
pub struct PagesProjectSpecDeploymentConfigsProductionMtlsCertificatesAdditionalProperty {
    #[serde(rename = "certificate_id")]
    pub certificate_id: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionMtlsCertificatesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionPlacement {
    ///Placement mode.
    #[serde(rename = "mode")]
    pub mode: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionQueueProducersAdditionalProperty {
    ///Name of the Queue.
    #[serde(rename = "name")]
    pub name: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionQueueProducersAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionR2BucketsAdditionalProperty {
    ///Jurisdiction of the R2 bucket.
    #[serde(rename = "jurisdiction", default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    ///Name of the R2 bucket.
    #[serde(rename = "name")]
    pub name: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionR2BucketsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment")]
    pub environment: String,
    ///The Service name.
    #[serde(rename = "service")]
    pub service: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionServicesAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProductionVectorizeBindingsAdditionalProperty {
    #[serde(rename = "index_name")]
    pub index_name: String,
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
pub struct PagesProjectSpecDeploymentConfigsProductionVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectSpecDeploymentConfigsProductionVectorizeBindingsAdditionalProperty,
    >,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigsProduction {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<PagesProjectSpecDeploymentConfigsProductionAiBindings>,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(rename = "always_use_latest_compatibility_date")]
    pub always_use_latest_compatibility_date: bool,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectSpecDeploymentConfigsProductionAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectSpecDeploymentConfigsProductionBrowsers>,
    ///The major version of the build image to use for Pages Functions.
    #[serde(rename = "build_image_major_version")]
    pub build_image_major_version: i64,
    ///Compatibility date used for Pages Functions.
    #[serde(rename = "compatibility_date")]
    pub compatibility_date: String,
    ///Compatibility flags used for Pages Functions.
    #[serde(rename = "compatibility_flags")]
    pub compatibility_flags: Vec<String>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<PagesProjectSpecDeploymentConfigsProductionD1Databases>,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectSpecDeploymentConfigsProductionDurableObjectNamespaces,
    >,
    #[serde(rename = "env_vars")]
    pub env_vars: PagesEnvVars,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open")]
    pub fail_open: bool,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectSpecDeploymentConfigsProductionHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<PagesProjectSpecDeploymentConfigsProductionKvNamespaces>,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectSpecDeploymentConfigsProductionLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectSpecDeploymentConfigsProductionMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectSpecDeploymentConfigsProductionPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectSpecDeploymentConfigsProductionQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectSpecDeploymentConfigsProductionR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectSpecDeploymentConfigsProductionServices>,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model")]
    pub usage_model: String,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectSpecDeploymentConfigsProductionVectorizeBindings,
    >,
    ///Hash of the Wrangler configuration used for the deployment.
    #[serde(
        rename = "wrangler_config_hash",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wrangler_config_hash: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectSpecDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(rename = "preview")]
    pub preview: PagesProjectSpecDeploymentConfigsPreview,
    ///Configs for production deploys.
    #[serde(rename = "production")]
    pub production: PagesProjectSpecDeploymentConfigsProduction,
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
pub struct PagesProjectSpecLatestDeploymentDeploymentTriggerMetadata {
    ///Where the trigger happened.
    #[serde(rename = "branch")]
    pub branch: String,
    ///Whether the deployment trigger commit was dirty.
    #[serde(rename = "commit_dirty")]
    pub commit_dirty: bool,
    ///Hash of the deployment trigger commit.
    #[serde(rename = "commit_hash")]
    pub commit_hash: String,
    ///Message of the deployment trigger commit.
    #[serde(rename = "commit_message")]
    pub commit_message: String,
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
pub struct PagesProjectSpecLatestDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectSpecLatestDeploymentDeploymentTriggerMetadata,
    ///What caused the deployment.
    #[serde(rename = "type")]
    pub r#type: String,
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
pub struct PagesProjectSpecLatestDeployment {
    ///A list of alias URLs pointing to this deployment.
    #[serde(rename = "aliases", default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "build_config")]
    pub build_config: PagesBuildConfig,
    ///When the deployment was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    ///Info about what caused the deployment.
    #[serde(rename = "deployment_trigger")]
    pub deployment_trigger: PagesProjectSpecLatestDeploymentDeploymentTrigger,
    #[serde(rename = "env_vars")]
    pub env_vars: PagesEnvVars,
    ///Type of deploy.
    #[serde(rename = "environment")]
    pub environment: String,
    ///Id of the deployment.
    #[serde(rename = "id")]
    pub id: String,
    ///If the deployment has been skipped.
    #[serde(rename = "is_skipped")]
    pub is_skipped: bool,
    #[serde(rename = "latest_stage")]
    pub latest_stage: PagesStage,
    ///When the deployment was last modified.
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    ///Id of the project.
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_name")]
    pub project_name: PagesProjectName,
    ///Short Id (8 character) of the deployment.
    #[serde(rename = "short_id")]
    pub short_id: String,
    ///Why the deployment was skipped.
    #[serde(rename = "skip_reason", default, skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<String>,
    #[serde(rename = "source")]
    pub source: PagesSource,
    ///List of past stages.
    #[serde(rename = "stages")]
    pub stages: Vec<PagesStage>,
    ///The live URL to view this deployment.
    #[serde(rename = "url")]
    pub url: String,
    ///Whether the deployment uses functions.
    #[serde(rename = "uses_functions", default, skip_serializing_if = "Option::is_none")]
    pub uses_functions: Option<bool>,
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
    kind = "PagesProject",
    namespaced,
    status = "PagesProjectStatus"
)]
pub struct PagesProjectSpec {
    #[serde(rename = "build_config", default, skip_serializing_if = "Option::is_none")]
    pub build_config: Option<PagesBuildConfig>,
    #[serde(rename = "canonical_deployment")]
    pub canonical_deployment: PagesProjectSpecCanonicalDeployment,
    ///When the project was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    ///Configs for deployments in a project.
    #[serde(rename = "deployment_configs")]
    pub deployment_configs: PagesProjectSpecDeploymentConfigs,
    ///A list of associated custom domains for the project.
    #[serde(rename = "domains", default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    ///Framework the project is using.
    #[serde(rename = "framework")]
    pub framework: String,
    ///Version of the framework the project is using.
    #[serde(rename = "framework_version")]
    pub framework_version: String,
    ///ID of the project.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "latest_deployment")]
    pub latest_deployment: PagesProjectSpecLatestDeployment,
    #[serde(rename = "name")]
    pub name: PagesProjectName,
    ///Name of the preview script.
    #[serde(rename = "preview_script_name")]
    pub preview_script_name: String,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(rename = "production_branch")]
    pub production_branch: String,
    ///Name of the production script.
    #[serde(rename = "production_script_name")]
    pub production_script_name: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesSource>,
    ///The Cloudflare subdomain associated with the project.
    #[serde(rename = "subdomain", default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    ///Whether the project uses functions.
    #[serde(rename = "uses_functions", default, skip_serializing_if = "Option::is_none")]
    pub uses_functions: Option<bool>,
}
