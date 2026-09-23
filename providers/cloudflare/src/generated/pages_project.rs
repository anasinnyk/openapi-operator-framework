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
pub struct PagesProjectForProviderCanonicalDeploymentDeploymentTriggerMetadata {
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
pub struct PagesProjectForProviderCanonicalDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectForProviderCanonicalDeploymentDeploymentTriggerMetadata,
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
pub struct PagesProjectForProviderCanonicalDeployment {
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
    pub deployment_trigger: PagesProjectForProviderCanonicalDeploymentDeploymentTrigger,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewAiBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewAiBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewBrowsersAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewBrowsersAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewD1DatabasesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewD1DatabasesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewKvNamespacesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewKvNamespacesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewLimits {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewPlacement {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewQueueProducersAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewQueueProducersAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewR2BucketsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewR2BucketsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewServicesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewServicesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsPreviewVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsPreview {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<PagesProjectForProviderDeploymentConfigsPreviewAiBindings>,
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
        PagesProjectForProviderDeploymentConfigsPreviewAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectForProviderDeploymentConfigsPreviewBrowsers>,
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
    pub d1_databases: Option<PagesProjectForProviderDeploymentConfigsPreviewD1Databases>,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectForProviderDeploymentConfigsPreviewDurableObjectNamespaces,
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
        PagesProjectForProviderDeploymentConfigsPreviewHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectForProviderDeploymentConfigsPreviewKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectForProviderDeploymentConfigsPreviewLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectForProviderDeploymentConfigsPreviewMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectForProviderDeploymentConfigsPreviewPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectForProviderDeploymentConfigsPreviewQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectForProviderDeploymentConfigsPreviewR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectForProviderDeploymentConfigsPreviewServices>,
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
        PagesProjectForProviderDeploymentConfigsPreviewVectorizeBindings,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionAiBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionAiBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionBrowsersAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionBrowsersAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionD1DatabasesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionD1DatabasesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionKvNamespacesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionKvNamespacesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionLimits {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionPlacement {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionQueueProducersAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionQueueProducersAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionR2BucketsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionR2BucketsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionServicesAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionServicesAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProductionVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectForProviderDeploymentConfigsProductionVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectForProviderDeploymentConfigsProductionVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectForProviderDeploymentConfigsProduction {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<
        PagesProjectForProviderDeploymentConfigsProductionAiBindings,
    >,
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
        PagesProjectForProviderDeploymentConfigsProductionAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectForProviderDeploymentConfigsProductionBrowsers>,
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
    pub d1_databases: Option<
        PagesProjectForProviderDeploymentConfigsProductionD1Databases,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectForProviderDeploymentConfigsProductionDurableObjectNamespaces,
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
        PagesProjectForProviderDeploymentConfigsProductionHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectForProviderDeploymentConfigsProductionKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectForProviderDeploymentConfigsProductionLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectForProviderDeploymentConfigsProductionMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectForProviderDeploymentConfigsProductionPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectForProviderDeploymentConfigsProductionQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectForProviderDeploymentConfigsProductionR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectForProviderDeploymentConfigsProductionServices>,
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
        PagesProjectForProviderDeploymentConfigsProductionVectorizeBindings,
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
pub struct PagesProjectForProviderDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(rename = "preview")]
    pub preview: PagesProjectForProviderDeploymentConfigsPreview,
    ///Configs for production deploys.
    #[serde(rename = "production")]
    pub production: PagesProjectForProviderDeploymentConfigsProduction,
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
pub struct PagesProjectForProviderLatestDeploymentDeploymentTriggerMetadata {
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
pub struct PagesProjectForProviderLatestDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectForProviderLatestDeploymentDeploymentTriggerMetadata,
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
pub struct PagesProjectForProviderLatestDeployment {
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
    pub deployment_trigger: PagesProjectForProviderLatestDeploymentDeploymentTrigger,
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
pub struct PagesProjectForProvider {
    #[serde(rename = "build_config", default, skip_serializing_if = "Option::is_none")]
    pub build_config: Option<PagesBuildConfig>,
    #[serde(rename = "canonical_deployment")]
    pub canonical_deployment: PagesProjectForProviderCanonicalDeployment,
    ///Configs for deployments in a project.
    #[serde(rename = "deployment_configs")]
    pub deployment_configs: PagesProjectForProviderDeploymentConfigs,
    #[serde(rename = "latest_deployment")]
    pub latest_deployment: PagesProjectForProviderLatestDeployment,
    #[serde(rename = "name")]
    pub name: PagesProjectName,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(rename = "production_branch")]
    pub production_branch: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesSource>,
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
    kind = "PagesProject",
    namespaced,
    status = "PagesProjectStatus"
)]
pub struct PagesProjectSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: PagesProjectForProvider,
    #[serde(flatten)]
    pub management: core::managed::ManagedResourceSpec,
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
pub struct PagesProjectAtProvider {
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
pub struct PagesProjectStatus {
    #[serde(rename = "atProvider", default, skip_serializing_if = "Option::is_none")]
    pub at_provider: Option<PagesProjectAtProvider>,
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
    resource: &PagesProject,
) -> Result<core::reconciler::Observation, core::error::ReconcileError> {
    let account_id = (async {
        let root_namespace = kube::ResourceExt::namespace(resource)
            .ok_or_else(|| {
                core::error::ResolveValueError::Missing(
                    "resource has no namespace".into(),
                )
            })?;
        let reference_0 = (resource.spec.for_provider.account_ref)
            .as_ref()
            .ok_or_else(|| {
                core::error::ResolveValueError::Missing(
                    format!(
                        "resource reference {} is not set",
                        "spec.forProvider.accountRef",
                    ),
                )
            })?;
        let namespace_0 = reference_0
            .namespace
            .clone()
            .unwrap_or_else(|| root_namespace.clone());
        let api_0: kube::Api<crate::generated::account::Account> = kube::Api::namespaced(
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
                format!("could not resolve path parameter {}", "account_id",),
            )
        })?;
    let project_name = (async {
        let mut resolved: Option<String> = None;
        if resolved.is_none() {
            resolved = (async {
                core::api::resolve_field_value(&resource, "status.atProvider.id")
            })
                .await?;
        }
        if resolved.is_none() {
            resolved = (async {
                core::api::resolve_field_value(&resource, "spec.resourceName")
            })
                .await?;
        }
        if resolved.is_none() {
            resolved = (async {
                core::api::resolve_field_value(&resource, "metadata.name")
            })
                .await?;
        }
        Ok::<Option<String>, core::error::ResolveValueError>(resolved)
    })
        .await?
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "project_name",),
            )
        })?;
    let request = provider_client.pages_project_get_project(&account_id, &project_name);
    let credentials = resolve_credentials(kube_client, resource).await?;
    let request = request.with_credentials(&credentials);
    let observed = request.send_optional().await?;
    let at_provider = observed.map(serde_json::to_value).transpose()?;
    Ok(core::reconciler::Observation {
        exists: at_provider.is_some(),
        at_provider,
    })
}
pub async fn resolve_credentials(
    client: &kube::Client,
    resource: &PagesProject,
) -> Result<crate::generated::client::ApiTokenCredential, core::api::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            core::api::CredentialError::MissingValue(
                format!("{} has no namespace", "PagesProject",),
            )
        })?;
    let reference_0 = (resource.spec.for_provider.account_ref)
        .as_ref()
        .ok_or_else(|| {
            core::error::CredentialError::MissingValue(
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
            core::api::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = core::reference::resolve_secret_key(client, &namespace_0, selector)
        .await?;
    Ok(crate::generated::client::ApiTokenCredential::new(value)?)
}
pub async fn update_status(
    client: &kube::Client,
    resource: &PagesProject,
    observation: &core::reconciler::Observation,
) -> Result<(), core::error::ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing("resource has no namespace".into())
        })?;
    let name = kube::ResourceExt::name_any(resource);
    let at_provider: Option<PagesProjectAtProvider> = observation
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
    let status = PagesProjectStatus {
        at_provider,
        observed_generation: resource.metadata.generation,
        conditions: vec![condition],
    };
    let patch = serde_json::json!({ "status" : status, });
    let api: kube::Api<PagesProject> = kube::Api::namespaced(client.clone(), &namespace);
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
    resource: std::sync::Arc<PagesProject>,
    context: std::sync::Arc<
        core::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) -> Result<kube::runtime::controller::Action, core::error::ReconcileError> {
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
    _resource: std::sync::Arc<PagesProject>,
    _error: &core::error::ReconcileError,
    _context: std::sync::Arc<
        core::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) -> kube::runtime::controller::Action {
    kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(30))
}
pub async fn run_controller(
    context: std::sync::Arc<
        core::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) {
    use futures::StreamExt as _;
    let api: kube::Api<PagesProject> = kube::Api::all(context.kube_client.clone());
    kube::runtime::Controller::new(api, kube::runtime::watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|result| async move {
            match result {
                Ok((object, action)) => {
                    tracing::debug!(
                        resource = "PagesProject", ? object, ? action,
                        "reconciliation completed",
                    );
                }
                Err(error) => {
                    tracing::error!(
                        resource = "PagesProject", ? error, "reconciliation failed",
                    );
                }
            }
        })
        .await;
}
