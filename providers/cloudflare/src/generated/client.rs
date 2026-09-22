pub type IamAccountType = serde_json::Value;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamCreateAccountUnit {
    ///Tenant unit ID
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
pub struct IamCreateAccount {
    ///Account name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IamAccountType>,
    ///information related to the tenant unit, and optionally, an id of the unit to create the account on. see https://developers.cloudflare.com/tenant/how-to/manage-accounts/
    #[serde(rename = "unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<IamCreateAccountUnit>,
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
pub struct IamMessages2itemSource {
    #[serde(rename = "pointer", default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
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
pub struct IamMessages2item {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(
        rename = "documentation_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub documentation_url: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<IamMessages2itemSource>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type IamMessages2 = Vec<IamMessages2item>;
pub type IamCommonComponentsSchemasIdentifier = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamAccountManagedBy {
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
pub struct IamAccountSettings {
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
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamAccount {
    ///Timestamp for the creation of the account
    #[serde(rename = "created_on", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "id")]
    pub id: IamCommonComponentsSchemasIdentifier,
    ///Parent container details
    #[serde(rename = "managed_by", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<IamAccountManagedBy>,
    ///Account name
    #[serde(rename = "name")]
    pub name: String,
    ///Account settings
    #[serde(rename = "settings", default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<IamAccountSettings>,
    #[serde(rename = "type")]
    pub r#type: IamAccountType,
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
pub struct IamResponseSingleAccount {
    #[serde(rename = "errors")]
    pub errors: IamMessages2,
    #[serde(rename = "messages")]
    pub messages: IamMessages2,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<IamAccount>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamApiResponseSingleIdResult {
    #[serde(rename = "id")]
    pub id: IamCommonComponentsSchemasIdentifier,
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
pub struct IamApiResponseSingleId {
    #[serde(rename = "errors")]
    pub errors: IamMessages2,
    #[serde(rename = "messages")]
    pub messages: IamMessages2,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<IamApiResponseSingleIdResult>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamComponentsSchemasAccountManagedBy {
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
pub struct IamComponentsSchemasAccountSettings {
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
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct IamComponentsSchemasAccount {
    ///Timestamp for the creation of the account
    #[serde(rename = "created_on", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "id")]
    pub id: IamCommonComponentsSchemasIdentifier,
    ///Parent container details
    #[serde(rename = "managed_by", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<IamComponentsSchemasAccountManagedBy>,
    ///Account name
    #[serde(rename = "name")]
    pub name: String,
    ///Account settings
    #[serde(rename = "settings", default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<IamComponentsSchemasAccountSettings>,
    #[serde(rename = "type")]
    pub r#type: IamAccountType,
}
pub type RulesetsRulesetKind = String;
pub type RulesetsRulesetPhase = String;
pub type RulesetsRuleAction = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsBlockRuleActionParametersResponse {
    ///The content to return.
    #[serde(rename = "content")]
    pub content: String,
    ///The type of the content to return.
    #[serde(rename = "content_type")]
    pub content_type: String,
    ///The status code to return.
    #[serde(rename = "status_code")]
    pub status_code: i64,
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
pub struct RulesetsBlockRuleActionParameters {
    ///The response to show when the block is applied.
    #[serde(rename = "response", default, skip_serializing_if = "Option::is_none")]
    pub response: Option<RulesetsBlockRuleActionParametersResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsRuleCategory = String;
pub type RulesetsRuleCategories = Vec<RulesetsRuleCategory>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRuleExposedCredentialCheck {
    ///An expression that selects the password used in the credentials check.
    #[serde(rename = "password_expression")]
    pub password_expression: String,
    ///An expression that selects the user ID used in the credentials check.
    #[serde(rename = "username_expression")]
    pub username_expression: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsRuleId = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRuleLogging {
    ///Whether to generate a log when the rule matches.
    #[serde(rename = "enabled")]
    pub enabled: bool,
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
pub struct RulesetsRuleRatelimit {
    ///Characteristics of the request on which the rate limit counter will be incremented.
    #[serde(rename = "characteristics")]
    pub characteristics: Vec<String>,
    ///An expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression.
    #[serde(
        rename = "counting_expression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counting_expression: Option<String>,
    ///Period of time in seconds after which the action will be disabled following its first execution.
    #[serde(
        rename = "mitigation_timeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mitigation_timeout: Option<i64>,
    ///Period in seconds over which the counter is being incremented.
    #[serde(rename = "period")]
    pub period: i64,
    ///The threshold of requests per period after which the action will be executed for the first time.
    #[serde(
        rename = "requests_per_period",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requests_per_period: Option<i64>,
    ///Whether counting is only performed when an origin is reached.
    #[serde(
        rename = "requests_to_origin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requests_to_origin: Option<bool>,
    ///The score threshold per period for which the action will be executed the first time.
    #[serde(
        rename = "score_per_period",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score_per_period: Option<i64>,
    ///A response header name provided by the origin, which contains the score to increment rate limit counter with.
    #[serde(
        rename = "score_response_header_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score_response_header_name: Option<String>,
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
pub struct RulesetsBlockRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsBlockRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsChallengeRuleActionParameters {
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
pub struct RulesetsChallengeRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsChallengeRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsCompressResponseRuleActionParametersAlgorithmsItem {
    ///Name of the compression algorithm to enable.
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
pub struct RulesetsCompressResponseRuleActionParameters {
    ///Custom order for compression algorithms.
    #[serde(rename = "algorithms")]
    pub algorithms: Vec<RulesetsCompressResponseRuleActionParametersAlgorithmsItem>,
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
pub struct RulesetsCompressResponseRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsCompressResponseRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsDDoSDynamicRuleActionParameters {
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
pub struct RulesetsDDoSDynamicRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsDDoSDynamicRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteMatchedData {
    ///The public key to encrypt matched data logs with.
    #[serde(rename = "public_key")]
    pub public_key: String,
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
pub struct RulesetsExecuteCategoryOverridesItem {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "sensitivity_level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sensitivity_level: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsExecuteCategoryOverrides = Vec<RulesetsExecuteCategoryOverridesItem>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteRuleOverridesItem {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    ///The score threshold to use for the rule.
    #[serde(
        rename = "score_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score_threshold: Option<i64>,
    #[serde(
        rename = "sensitivity_level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sensitivity_level: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsExecuteRuleOverrides = Vec<RulesetsExecuteRuleOverridesItem>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteOverrides {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsExecuteCategoryOverrides>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RulesetsExecuteRuleOverrides>,
    #[serde(
        rename = "sensitivity_level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sensitivity_level: Option<String>,
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
pub struct RulesetsExecuteRuleActionParameters {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "matched_data", default, skip_serializing_if = "Option::is_none")]
    pub matched_data: Option<RulesetsExecuteMatchedData>,
    #[serde(rename = "overrides", default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<RulesetsExecuteOverrides>,
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
pub struct RulesetsExecuteRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsExecuteRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsForceConnectionCloseRuleActionParameters {
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
pub struct RulesetsForceConnectionCloseRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsForceConnectionCloseRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsJsChallengeRuleActionParameters {
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
pub struct RulesetsJsChallengeRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsJsChallengeRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogRuleActionParameters {
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
pub struct RulesetsLogRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsLogRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldCookieFieldsItem {
    ///The name of the cookie.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsLogCustomFieldCookieFields = Vec<
    RulesetsLogCustomFieldCookieFieldsItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldRawResponseFieldsItem {
    ///The name of the response header.
    #[serde(rename = "name")]
    pub name: String,
    ///Whether to log duplicate values of the same header.
    #[serde(
        rename = "preserve_duplicates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preserve_duplicates: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsLogCustomFieldRawResponseFields = Vec<
    RulesetsLogCustomFieldRawResponseFieldsItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldRequestFieldsItem {
    ///The name of the header.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsLogCustomFieldRequestFields = Vec<
    RulesetsLogCustomFieldRequestFieldsItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldResponseFieldsItem {
    ///The name of the response header.
    #[serde(rename = "name")]
    pub name: String,
    ///Whether to log duplicate values of the same header.
    #[serde(
        rename = "preserve_duplicates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preserve_duplicates: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsLogCustomFieldResponseFields = Vec<
    RulesetsLogCustomFieldResponseFieldsItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldTransformedRequestFieldsItem {
    ///The name of the header.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsLogCustomFieldTransformedRequestFields = Vec<
    RulesetsLogCustomFieldTransformedRequestFieldsItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldRuleActionParameters {
    #[serde(rename = "cookie_fields", default, skip_serializing_if = "Option::is_none")]
    pub cookie_fields: Option<RulesetsLogCustomFieldCookieFields>,
    #[serde(
        rename = "raw_response_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_response_fields: Option<RulesetsLogCustomFieldRawResponseFields>,
    #[serde(rename = "request_fields", default, skip_serializing_if = "Option::is_none")]
    pub request_fields: Option<RulesetsLogCustomFieldRequestFields>,
    #[serde(
        rename = "response_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_fields: Option<RulesetsLogCustomFieldResponseFields>,
    #[serde(
        rename = "transformed_request_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transformed_request_fields: Option<
        RulesetsLogCustomFieldTransformedRequestFields,
    >,
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
pub struct RulesetsLogCustomFieldRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsLogCustomFieldRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsManagedChallengeRuleActionParameters {
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
pub struct RulesetsManagedChallengeRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsManagedChallengeRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRedirectFromList {
    ///An expression that evaluates to the list lookup key.
    #[serde(rename = "key")]
    pub key: String,
    ///The name of the list to match against.
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
pub struct RulesetsRedirectFromValueTargetUrl {
    ///An expression that evaluates to a URL to redirect the request to.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    ///A URL to redirect the request to.
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
pub struct RulesetsRedirectFromValue {
    ///Whether to keep the query string of the original request.
    #[serde(
        rename = "preserve_query_string",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preserve_query_string: Option<bool>,
    ///The status code to use for the redirect.
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    ///A URL to redirect the request to.
    #[serde(rename = "target_url")]
    pub target_url: RulesetsRedirectFromValueTargetUrl,
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
pub struct RulesetsRedirectRuleActionParameters {
    #[serde(rename = "from_list", default, skip_serializing_if = "Option::is_none")]
    pub from_list: Option<RulesetsRedirectFromList>,
    #[serde(rename = "from_value", default, skip_serializing_if = "Option::is_none")]
    pub from_value: Option<RulesetsRedirectFromValue>,
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
pub struct RulesetsRedirectRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsRedirectRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsRewriteHeaderValue = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteHeadersAdditionalPropertyVariant1 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "value")]
    pub value: RulesetsRewriteHeaderValue,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsRewriteHeaderExpression = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteHeadersAdditionalPropertyVariant2 {
    #[serde(rename = "expression")]
    pub expression: RulesetsRewriteHeaderExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsRewriteHeadersAdditionalPropertyVariant3 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "value")]
    pub value: RulesetsRewriteHeaderValue,
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
pub struct RulesetsRewriteHeadersAdditionalPropertyVariant4 {
    #[serde(rename = "expression")]
    pub expression: RulesetsRewriteHeaderExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsRewriteHeadersAdditionalPropertyVariant5 {
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsRewriteHeadersAdditionalProperty {
    Variant1(RulesetsRewriteHeadersAdditionalPropertyVariant1),
    Variant2(RulesetsRewriteHeadersAdditionalPropertyVariant2),
    Variant3(RulesetsRewriteHeadersAdditionalPropertyVariant3),
    Variant4(RulesetsRewriteHeadersAdditionalPropertyVariant4),
    Variant5(RulesetsRewriteHeadersAdditionalPropertyVariant5),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteHeaders {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        RulesetsRewriteHeadersAdditionalProperty,
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
pub struct RulesetsRewriteUri {
    ///Whether to propagate the rewritten URI to origin.
    #[serde(rename = "origin", default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteRuleActionParameters {
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<RulesetsRewriteHeaders>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<RulesetsRewriteUri>,
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
pub struct RulesetsRewriteRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsRewriteRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsRouteHostHeader = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsRouteOrigin {
    ///A resolved host to route to.
    #[serde(rename = "host", default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    ///A destination port to route to.
    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
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
pub struct RulesetsRouteSni {
    ///A value to override the SNI to.
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
pub struct RulesetsRouteRuleActionParameters {
    #[serde(rename = "host_header", default, skip_serializing_if = "Option::is_none")]
    pub host_header: Option<RulesetsRouteHostHeader>,
    #[serde(rename = "origin", default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<RulesetsRouteOrigin>,
    #[serde(rename = "sni", default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<RulesetsRouteSni>,
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
pub struct RulesetsRouteRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsRouteRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsScoreIncrement = i64;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsScoreRuleActionParameters {
    #[serde(rename = "increment")]
    pub increment: RulesetsScoreIncrement,
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
pub struct RulesetsScoreRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsScoreRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsServeErrorContent = String;
pub type RulesetsServeErrorContentType = String;
pub type RulesetsServeErrorStatusCode = i64;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsServeErrorRuleActionParametersVariant1 {
    #[serde(rename = "content")]
    pub content: RulesetsServeErrorContent,
    #[serde(rename = "content_type")]
    pub content_type: RulesetsServeErrorContentType,
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RulesetsServeErrorStatusCode>,
}
pub type RulesetsServeErrorAssetName = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsServeErrorRuleActionParametersVariant2 {
    #[serde(rename = "asset_name")]
    pub asset_name: RulesetsServeErrorAssetName,
    #[serde(rename = "content_type")]
    pub content_type: RulesetsServeErrorContentType,
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RulesetsServeErrorStatusCode>,
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
pub enum RulesetsServeErrorRuleActionParameters {
    Variant1(RulesetsServeErrorRuleActionParametersVariant1),
    Variant2(RulesetsServeErrorRuleActionParametersVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsServeErrorRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsServeErrorRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsSetCacheControlCloudflareOnly = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlDirectiveVariant1 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsSetCacheControlDirectiveVariant2 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsSetCacheControlDirective {
    Variant1(RulesetsSetCacheControlDirectiveVariant1),
    Variant2(RulesetsSetCacheControlDirectiveVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlDirectiveWithValueVariant1 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
    ///The duration value in seconds for the directive.
    #[serde(rename = "value")]
    pub value: i64,
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
pub struct RulesetsSetCacheControlDirectiveWithValueVariant2 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsSetCacheControlDirectiveWithValue {
    Variant1(RulesetsSetCacheControlDirectiveWithValueVariant1),
    Variant2(RulesetsSetCacheControlDirectiveWithValueVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlDirectiveWithQualifiersVariant1 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
    ///Optional list of header names to qualify the directive (e.g., for "private" or "no-cache" directives).
    #[serde(rename = "qualifiers", default, skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<String>>,
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
pub struct RulesetsSetCacheControlDirectiveWithQualifiersVariant2 {
    #[serde(
        rename = "cloudflare_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloudflare_only: Option<RulesetsSetCacheControlCloudflareOnly>,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsSetCacheControlDirectiveWithQualifiers {
    Variant1(RulesetsSetCacheControlDirectiveWithQualifiersVariant1),
    Variant2(RulesetsSetCacheControlDirectiveWithQualifiersVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlRuleActionParameters {
    #[serde(rename = "immutable", default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "max-age", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "must-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub must_revalidate: Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "must-understand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub must_understand: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-cache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(rename = "no-store", default, skip_serializing_if = "Option::is_none")]
    pub no_store: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-transform", default, skip_serializing_if = "Option::is_none")]
    pub no_transform: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "private", default, skip_serializing_if = "Option::is_none")]
    pub private: Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(
        rename = "proxy-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_revalidate: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "public", default, skip_serializing_if = "Option::is_none")]
    pub public: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "s-maxage", default, skip_serializing_if = "Option::is_none")]
    pub s_maxage: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(rename = "stale-if-error", default, skip_serializing_if = "Option::is_none")]
    pub stale_if_error: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-while-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stale_while_revalidate: Option<RulesetsSetCacheControlDirectiveWithValue>,
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
pub struct RulesetsSetCacheControlRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsSetCacheControlRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsSetCacheSettingsAdditionalCacheablePorts = Vec<i64>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsBrowserTtl {
    ///The browser TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    ///The browser TTL mode.
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheSettingsCache = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyCookie {
    ///A list of cookies to check for the presence of. The presence of these cookies is included in the cache key.
    #[serde(rename = "check_presence", default, skip_serializing_if = "Option::is_none")]
    pub check_presence: Option<Vec<String>>,
    ///A list of cookies to include in the cache key.
    #[serde(rename = "include", default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyHeaderContains {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyHeader {
    ///A list of headers to check for the presence of. The presence of these headers is included in the cache key.
    #[serde(rename = "check_presence", default, skip_serializing_if = "Option::is_none")]
    pub check_presence: Option<Vec<String>>,
    ///A mapping of header names to a list of values. If a header is present in the request and contains any of the values provided, its value is included in the cache key.
    #[serde(rename = "contains", default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<RulesetsSetCacheSettingsCustomCacheKeyHeaderContains>,
    ///Whether to exclude the origin header in the cache key.
    #[serde(rename = "exclude_origin", default, skip_serializing_if = "Option::is_none")]
    pub exclude_origin: Option<bool>,
    ///A list of headers to include in the cache key.
    #[serde(rename = "include", default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyHost {
    ///Whether to use the resolved host in the cache key.
    #[serde(rename = "resolved", default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyQueryStringExclude {
    ///Whether to exclude all query string parameters from the cache key.
    #[serde(rename = "all", default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    ///A list of query string parameters to exclude from the cache key.
    #[serde(rename = "list", default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyQueryStringInclude {
    ///Whether to include all query string parameters in the cache key.
    #[serde(rename = "all", default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    ///A list of query string parameters to include in the cache key.
    #[serde(rename = "list", default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyQueryString {
    ///Which query string parameters to exclude from the cache key.
    #[serde(rename = "exclude", default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<RulesetsSetCacheSettingsCustomCacheKeyQueryStringExclude>,
    ///Which query string parameters to include in the cache key.
    #[serde(rename = "include", default, skip_serializing_if = "Option::is_none")]
    pub include: Option<RulesetsSetCacheSettingsCustomCacheKeyQueryStringInclude>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKeyUser {
    ///Whether to use the user agent's device type in the cache key.
    #[serde(rename = "device_type", default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<bool>,
    ///Whether to use the user agents's country in the cache key.
    #[serde(rename = "geo", default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
    ///Whether to use the user agent's language in the cache key.
    #[serde(rename = "lang", default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<bool>,
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
pub struct RulesetsSetCacheSettingsCustomCacheKey {
    #[serde(rename = "cookie", default, skip_serializing_if = "Option::is_none")]
    pub cookie: Option<RulesetsSetCacheSettingsCustomCacheKeyCookie>,
    #[serde(rename = "header", default, skip_serializing_if = "Option::is_none")]
    pub header: Option<RulesetsSetCacheSettingsCustomCacheKeyHeader>,
    #[serde(rename = "host", default, skip_serializing_if = "Option::is_none")]
    pub host: Option<RulesetsSetCacheSettingsCustomCacheKeyHost>,
    #[serde(rename = "query_string", default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<RulesetsSetCacheSettingsCustomCacheKeyQueryString>,
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<RulesetsSetCacheSettingsCustomCacheKeyUser>,
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
pub struct RulesetsSetCacheSettingsCacheKey {
    ///Whether to separate cached content based on the visitor's device type.
    #[serde(
        rename = "cache_by_device_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_by_device_type: Option<bool>,
    ///Whether to protect from web cache deception attacks, while allowing static assets to be cached.
    #[serde(
        rename = "cache_deception_armor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_deception_armor: Option<bool>,
    #[serde(rename = "custom_key", default, skip_serializing_if = "Option::is_none")]
    pub custom_key: Option<RulesetsSetCacheSettingsCustomCacheKey>,
    ///Whether to treat requests with the same query parameters the same, regardless of the order those query parameters are in.
    #[serde(
        rename = "ignore_query_strings_order",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_query_strings_order: Option<bool>,
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
pub struct RulesetsSetCacheSettingsCacheReserve {
    ///Whether Cache Reserve is enabled. If this is true and a request meets eligibility criteria, Cloudflare will write the resource to Cache Reserve.
    #[serde(rename = "eligible")]
    pub eligible: bool,
    ///The minimum file size eligible for storage in Cache Reserve.
    #[serde(
        rename = "minimum_file_size",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_file_size: Option<i64>,
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
pub struct RulesetsSetCacheSettingsStatusCodeTtlItemStatusCodeRange {
    ///The lower bound of the range.
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    ///The upper bound of the range.
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
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
pub struct RulesetsSetCacheSettingsStatusCodeTtlItem {
    ///A single status code to apply the TTL to.
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    ///A range of status codes to apply the TTL to.
    #[serde(
        rename = "status_code_range",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code_range: Option<
        RulesetsSetCacheSettingsStatusCodeTtlItemStatusCodeRange,
    >,
    ///The time to cache the response for (in seconds). A value of 0 is equivalent to setting the cache control header with the value "no-cache". A value of -1 is equivalent to setting the cache control header with the value of "no-store".
    #[serde(rename = "value")]
    pub value: i64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheSettingsStatusCodeTtl = Vec<
    RulesetsSetCacheSettingsStatusCodeTtlItem,
>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsEdgeTtl {
    ///The edge TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    ///The edge TTL mode.
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(
        rename = "status_code_ttl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code_ttl: Option<RulesetsSetCacheSettingsStatusCodeTtl>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheSettingsOriginCacheControl = bool;
pub type RulesetsSetCacheSettingsOriginErrorPagePassthru = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsOriginRangeRequests {
    ///Whether to use range requests. `default` is the behaviour the zone gets without this rule.
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheSettingsReadTimeout = i64;
pub type RulesetsSetCacheSettingsRespectStrongEtags = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsServeStale {
    ///Whether Cloudflare should disable serving stale content while getting the latest content from the origin.
    #[serde(
        rename = "disable_stale_while_updating",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_stale_while_updating: Option<bool>,
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
pub struct RulesetsSetCacheSettingsSharedDictionary {
    ///URL pattern for the Use-As-Dictionary match field. This pattern specifies which URLs can use this response as a dictionary.
    #[serde(rename = "match_pattern")]
    pub match_pattern: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheSettingsStripETags = bool;
pub type RulesetsSetCacheSettingsStripLastModified = bool;
pub type RulesetsSetCacheSettingsStripSetCookie = bool;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsVaryDefault {
    ///How the header value is treated when building the cache key.
    #[serde(rename = "action")]
    pub action: String,
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
pub struct RulesetsSetCacheSettingsVaryHeader {
    ///How the header value is treated when building the cache key.
    #[serde(rename = "action")]
    pub action: String,
    ///The set of languages to normalize against. Only valid for the `accept-language` header.
    #[serde(rename = "languages", default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    ///The set of media types to normalize against. Only valid for the `accept` header.
    #[serde(rename = "media_types", default, skip_serializing_if = "Option::is_none")]
    pub media_types: Option<Vec<String>>,
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
pub struct RulesetsSetCacheSettingsVaryHeaders {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        RulesetsSetCacheSettingsVaryHeader,
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
pub struct RulesetsSetCacheSettingsVary {
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<RulesetsSetCacheSettingsVaryDefault>,
    ///A mapping of lowercase request header names to their vary configuration.
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<RulesetsSetCacheSettingsVaryHeaders>,
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
pub struct RulesetsSetCacheSettingsRuleActionParameters {
    #[serde(
        rename = "additional_cacheable_ports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_cacheable_ports: Option<
        RulesetsSetCacheSettingsAdditionalCacheablePorts,
    >,
    #[serde(rename = "browser_ttl", default, skip_serializing_if = "Option::is_none")]
    pub browser_ttl: Option<RulesetsSetCacheSettingsBrowserTtl>,
    #[serde(rename = "cache", default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<RulesetsSetCacheSettingsCache>,
    #[serde(rename = "cache_key", default, skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<RulesetsSetCacheSettingsCacheKey>,
    #[serde(rename = "cache_reserve", default, skip_serializing_if = "Option::is_none")]
    pub cache_reserve: Option<RulesetsSetCacheSettingsCacheReserve>,
    #[serde(rename = "edge_ttl", default, skip_serializing_if = "Option::is_none")]
    pub edge_ttl: Option<RulesetsSetCacheSettingsEdgeTtl>,
    #[serde(
        rename = "origin_cache_control",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_cache_control: Option<RulesetsSetCacheSettingsOriginCacheControl>,
    #[serde(
        rename = "origin_error_page_passthru",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_error_page_passthru: Option<
        RulesetsSetCacheSettingsOriginErrorPagePassthru,
    >,
    #[serde(
        rename = "origin_range_requests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_range_requests: Option<RulesetsSetCacheSettingsOriginRangeRequests>,
    #[serde(rename = "read_timeout", default, skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<RulesetsSetCacheSettingsReadTimeout>,
    #[serde(
        rename = "respect_strong_etags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub respect_strong_etags: Option<RulesetsSetCacheSettingsRespectStrongEtags>,
    #[serde(rename = "serve_stale", default, skip_serializing_if = "Option::is_none")]
    pub serve_stale: Option<RulesetsSetCacheSettingsServeStale>,
    #[serde(
        rename = "shared_dictionary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_dictionary: Option<RulesetsSetCacheSettingsSharedDictionary>,
    #[serde(rename = "strip_etags", default, skip_serializing_if = "Option::is_none")]
    pub strip_etags: Option<RulesetsSetCacheSettingsStripETags>,
    #[serde(
        rename = "strip_last_modified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strip_last_modified: Option<RulesetsSetCacheSettingsStripLastModified>,
    #[serde(
        rename = "strip_set_cookie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strip_set_cookie: Option<RulesetsSetCacheSettingsStripSetCookie>,
    #[serde(rename = "vary", default, skip_serializing_if = "Option::is_none")]
    pub vary: Option<RulesetsSetCacheSettingsVary>,
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
pub struct RulesetsSetCacheSettingsRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsSetCacheSettingsRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsSetCacheTagsValues = Vec<String>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheTagsRuleActionParametersVariant1 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsSetCacheTagsExpression = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheTagsRuleActionParametersVariant2 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsSetCacheTagsRuleActionParametersVariant3 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
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
pub struct RulesetsSetCacheTagsRuleActionParametersVariant4 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsSetCacheTagsRuleActionParametersVariant5 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
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
pub struct RulesetsSetCacheTagsRuleActionParametersVariant6 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsSetCacheTagsRuleActionParameters {
    Variant1(RulesetsSetCacheTagsRuleActionParametersVariant1),
    Variant2(RulesetsSetCacheTagsRuleActionParametersVariant2),
    Variant3(RulesetsSetCacheTagsRuleActionParametersVariant3),
    Variant4(RulesetsSetCacheTagsRuleActionParametersVariant4),
    Variant5(RulesetsSetCacheTagsRuleActionParametersVariant5),
    Variant6(RulesetsSetCacheTagsRuleActionParametersVariant6),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheTagsRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsSetCacheTagsRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSetConfigAutominify {
    ///Whether to minify CSS files.
    #[serde(rename = "css", default, skip_serializing_if = "Option::is_none")]
    pub css: Option<bool>,
    ///Whether to minify HTML files.
    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<bool>,
    ///Whether to minify JavaScript files.
    #[serde(rename = "js", default, skip_serializing_if = "Option::is_none")]
    pub js: Option<bool>,
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
pub struct RulesetsSetConfigRuleActionParameters {
    ///Whether to enable Automatic HTTPS Rewrites.
    #[serde(
        rename = "automatic_https_rewrites",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_https_rewrites: Option<bool>,
    #[serde(rename = "autominify", default, skip_serializing_if = "Option::is_none")]
    pub autominify: Option<RulesetsSetConfigAutominify>,
    ///Whether to enable Browser Integrity Check (BIC).
    #[serde(rename = "bic", default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<bool>,
    ///Whether to enable content conversion (e.g., HTML to Markdown).
    #[serde(
        rename = "content_converter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_converter: Option<bool>,
    ///Whether to disable Cloudflare Apps.
    #[serde(rename = "disable_apps", default, skip_serializing_if = "Option::is_none")]
    pub disable_apps: Option<bool>,
    ///Whether to disable Pay Per Crawl.
    #[serde(
        rename = "disable_pay_per_crawl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_pay_per_crawl: Option<bool>,
    ///Whether to disable Real User Monitoring (RUM).
    #[serde(rename = "disable_rum", default, skip_serializing_if = "Option::is_none")]
    pub disable_rum: Option<bool>,
    ///Whether to disable Zaraz.
    #[serde(rename = "disable_zaraz", default, skip_serializing_if = "Option::is_none")]
    pub disable_zaraz: Option<bool>,
    ///Whether to enable Email Obfuscation.
    #[serde(
        rename = "email_obfuscation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_obfuscation: Option<bool>,
    ///Whether to enable Cloudflare Fonts.
    #[serde(rename = "fonts", default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<bool>,
    ///Whether to enable Hotlink Protection.
    #[serde(
        rename = "hotlink_protection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hotlink_protection: Option<bool>,
    ///Whether to enable Mirage.
    #[serde(rename = "mirage", default, skip_serializing_if = "Option::is_none")]
    pub mirage: Option<bool>,
    ///Whether to enable Opportunistic Encryption.
    #[serde(
        rename = "opportunistic_encryption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub opportunistic_encryption: Option<bool>,
    ///The Polish level to configure.
    #[serde(rename = "polish", default, skip_serializing_if = "Option::is_none")]
    pub polish: Option<String>,
    ///Whether to redirect verified AI training crawlers to canonical URLs found in the HTML response.
    #[serde(
        rename = "redirects_for_ai_training",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redirects_for_ai_training: Option<bool>,
    ///The request body buffering mode.
    #[serde(
        rename = "request_body_buffering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_body_buffering: Option<String>,
    ///The response body buffering mode.
    #[serde(
        rename = "response_body_buffering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_body_buffering: Option<String>,
    ///Whether to enable Rocket Loader.
    #[serde(rename = "rocket_loader", default, skip_serializing_if = "Option::is_none")]
    pub rocket_loader: Option<bool>,
    ///The Security Level to configure.
    #[serde(rename = "security_level", default, skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,
    ///Whether to enable Server-Side Excludes.
    #[serde(
        rename = "server_side_excludes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_excludes: Option<bool>,
    ///The SSL level to configure.
    #[serde(rename = "ssl", default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,
    ///Whether to enable Signed Exchanges (SXG).
    #[serde(rename = "sxg", default, skip_serializing_if = "Option::is_none")]
    pub sxg: Option<bool>,
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
pub struct RulesetsSetConfigRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsSetConfigRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
pub type RulesetsSkipPhase = String;
pub type RulesetsSkipPhases = Vec<String>;
pub type RulesetsSkipProducts = Vec<String>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSkipRules {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, Vec<String>>,
}
pub type RulesetsSkipRuleset = String;
pub type RulesetsSkipRulesets = Vec<String>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsSkipRuleActionParameters {
    #[serde(rename = "phase", default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<RulesetsSkipPhase>,
    #[serde(rename = "phases", default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<RulesetsSkipPhases>,
    #[serde(rename = "products", default, skip_serializing_if = "Option::is_none")]
    pub products: Option<RulesetsSkipProducts>,
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RulesetsSkipRules>,
    #[serde(rename = "ruleset", default, skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<RulesetsSkipRuleset>,
    #[serde(rename = "rulesets", default, skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<RulesetsSkipRulesets>,
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
pub struct RulesetsSkipRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsSkipRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsTransformResponseHtmlRuleActionParametersLinkMaze {
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
pub struct RulesetsTransformResponseHtmlRuleActionParameters {
    ///Enables the link maze transformation on the response.
    #[serde(rename = "link_maze")]
    pub link_maze: RulesetsTransformResponseHtmlRuleActionParametersLinkMaze,
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
pub struct RulesetsTransformResponseHtmlRule {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsTransformResponseHtmlRuleActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
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
pub enum RulesetsRequestRule {
    Variant1(RulesetsBlockRule),
    Variant2(RulesetsChallengeRule),
    Variant3(RulesetsCompressResponseRule),
    Variant4(RulesetsDDoSDynamicRule),
    Variant5(RulesetsExecuteRule),
    Variant6(RulesetsForceConnectionCloseRule),
    Variant7(RulesetsJsChallengeRule),
    Variant8(RulesetsLogRule),
    Variant9(RulesetsLogCustomFieldRule),
    Variant10(RulesetsManagedChallengeRule),
    Variant11(RulesetsRedirectRule),
    Variant12(RulesetsRewriteRule),
    Variant13(RulesetsRouteRule),
    Variant14(RulesetsScoreRule),
    Variant15(RulesetsServeErrorRule),
    Variant16(RulesetsSetCacheControlRule),
    Variant17(RulesetsSetCacheSettingsRule),
    Variant18(RulesetsSetCacheTagsRule),
    Variant19(RulesetsSetConfigRule),
    Variant20(RulesetsSkipRule),
    Variant21(RulesetsTransformResponseHtmlRule),
}
pub type RulesetsRequestRules = Vec<RulesetsRequestRule>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct CreateZoneRulesetRequest {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase")]
    pub phase: RulesetsRulesetPhase,
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RulesetsRequestRules>,
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsMessageSource {
    ///A JSON pointer to the field that is the source of the message.
    #[serde(rename = "pointer")]
    pub pointer: String,
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
pub struct RulesetsMessage {
    ///A unique code for this message.
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    ///A text description of this message.
    #[serde(rename = "message")]
    pub message: String,
    ///The source of this message.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<RulesetsMessageSource>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type RulesetsErrors = Vec<RulesetsMessage>;
pub type RulesetsMessages = Vec<RulesetsMessage>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant1ActionParametersResponse {
    ///The content to return.
    #[serde(rename = "content")]
    pub content: String,
    ///The type of the content to return.
    #[serde(rename = "content_type")]
    pub content_type: String,
    ///The status code to return.
    #[serde(rename = "status_code")]
    pub status_code: i64,
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
pub struct RulesetsResponseRuleVariant1ActionParameters {
    ///The response to show when the block is applied.
    #[serde(rename = "response", default, skip_serializing_if = "Option::is_none")]
    pub response: Option<RulesetsResponseRuleVariant1ActionParametersResponse>,
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
pub struct RulesetsResponseRuleVariant1 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant1ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant2ActionParameters {
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
pub struct RulesetsResponseRuleVariant2 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant2ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant3ActionParametersAlgorithmsItem {
    ///Name of the compression algorithm to enable.
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
pub struct RulesetsResponseRuleVariant3ActionParameters {
    ///Custom order for compression algorithms.
    #[serde(rename = "algorithms")]
    pub algorithms: Vec<RulesetsResponseRuleVariant3ActionParametersAlgorithmsItem>,
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
pub struct RulesetsResponseRuleVariant3 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant3ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant4ActionParameters {
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
pub struct RulesetsResponseRuleVariant4 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant4ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant5ActionParameters {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "matched_data", default, skip_serializing_if = "Option::is_none")]
    pub matched_data: Option<RulesetsExecuteMatchedData>,
    #[serde(rename = "overrides", default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<RulesetsExecuteOverrides>,
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
pub struct RulesetsResponseRuleVariant5 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant5ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant6ActionParameters {
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
pub struct RulesetsResponseRuleVariant6 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant6ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant7ActionParameters {
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
pub struct RulesetsResponseRuleVariant7 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant7ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant8ActionParameters {
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
pub struct RulesetsResponseRuleVariant8 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant8ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant9ActionParameters {
    #[serde(rename = "cookie_fields", default, skip_serializing_if = "Option::is_none")]
    pub cookie_fields: Option<RulesetsLogCustomFieldCookieFields>,
    #[serde(
        rename = "raw_response_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_response_fields: Option<RulesetsLogCustomFieldRawResponseFields>,
    #[serde(rename = "request_fields", default, skip_serializing_if = "Option::is_none")]
    pub request_fields: Option<RulesetsLogCustomFieldRequestFields>,
    #[serde(
        rename = "response_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_fields: Option<RulesetsLogCustomFieldResponseFields>,
    #[serde(
        rename = "transformed_request_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transformed_request_fields: Option<
        RulesetsLogCustomFieldTransformedRequestFields,
    >,
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
pub struct RulesetsResponseRuleVariant9 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant9ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant10ActionParameters {
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
pub struct RulesetsResponseRuleVariant10 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant10ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant11ActionParameters {
    #[serde(rename = "from_list", default, skip_serializing_if = "Option::is_none")]
    pub from_list: Option<RulesetsRedirectFromList>,
    #[serde(rename = "from_value", default, skip_serializing_if = "Option::is_none")]
    pub from_value: Option<RulesetsRedirectFromValue>,
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
pub struct RulesetsResponseRuleVariant11 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant11ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant12ActionParameters {
    #[serde(rename = "headers", default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<RulesetsRewriteHeaders>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<RulesetsRewriteUri>,
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
pub struct RulesetsResponseRuleVariant12 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant12ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant13ActionParameters {
    #[serde(rename = "host_header", default, skip_serializing_if = "Option::is_none")]
    pub host_header: Option<RulesetsRouteHostHeader>,
    #[serde(rename = "origin", default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<RulesetsRouteOrigin>,
    #[serde(rename = "sni", default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<RulesetsRouteSni>,
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
pub struct RulesetsResponseRuleVariant13 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant13ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant14ActionParameters {
    #[serde(rename = "increment")]
    pub increment: RulesetsScoreIncrement,
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
pub struct RulesetsResponseRuleVariant14 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant14ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant15ActionParametersVariant1 {
    #[serde(rename = "content")]
    pub content: RulesetsServeErrorContent,
    #[serde(rename = "content_type")]
    pub content_type: RulesetsServeErrorContentType,
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RulesetsServeErrorStatusCode>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant15ActionParametersVariant2 {
    #[serde(rename = "asset_name")]
    pub asset_name: RulesetsServeErrorAssetName,
    #[serde(rename = "content_type")]
    pub content_type: RulesetsServeErrorContentType,
    #[serde(rename = "status_code", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RulesetsServeErrorStatusCode>,
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
pub enum RulesetsResponseRuleVariant15ActionParameters {
    Variant1(RulesetsResponseRuleVariant15ActionParametersVariant1),
    Variant2(RulesetsResponseRuleVariant15ActionParametersVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant15 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant15ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant16ActionParameters {
    #[serde(rename = "immutable", default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "max-age", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "must-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub must_revalidate: Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "must-understand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub must_understand: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-cache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(rename = "no-store", default, skip_serializing_if = "Option::is_none")]
    pub no_store: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-transform", default, skip_serializing_if = "Option::is_none")]
    pub no_transform: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "private", default, skip_serializing_if = "Option::is_none")]
    pub private: Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(
        rename = "proxy-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_revalidate: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "public", default, skip_serializing_if = "Option::is_none")]
    pub public: Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "s-maxage", default, skip_serializing_if = "Option::is_none")]
    pub s_maxage: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(rename = "stale-if-error", default, skip_serializing_if = "Option::is_none")]
    pub stale_if_error: Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-while-revalidate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stale_while_revalidate: Option<RulesetsSetCacheControlDirectiveWithValue>,
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
pub struct RulesetsResponseRuleVariant16 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant16ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant17ActionParameters {
    #[serde(
        rename = "additional_cacheable_ports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_cacheable_ports: Option<
        RulesetsSetCacheSettingsAdditionalCacheablePorts,
    >,
    #[serde(rename = "browser_ttl", default, skip_serializing_if = "Option::is_none")]
    pub browser_ttl: Option<RulesetsSetCacheSettingsBrowserTtl>,
    #[serde(rename = "cache", default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<RulesetsSetCacheSettingsCache>,
    #[serde(rename = "cache_key", default, skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<RulesetsSetCacheSettingsCacheKey>,
    #[serde(rename = "cache_reserve", default, skip_serializing_if = "Option::is_none")]
    pub cache_reserve: Option<RulesetsSetCacheSettingsCacheReserve>,
    #[serde(rename = "edge_ttl", default, skip_serializing_if = "Option::is_none")]
    pub edge_ttl: Option<RulesetsSetCacheSettingsEdgeTtl>,
    #[serde(
        rename = "origin_cache_control",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_cache_control: Option<RulesetsSetCacheSettingsOriginCacheControl>,
    #[serde(
        rename = "origin_error_page_passthru",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_error_page_passthru: Option<
        RulesetsSetCacheSettingsOriginErrorPagePassthru,
    >,
    #[serde(
        rename = "origin_range_requests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_range_requests: Option<RulesetsSetCacheSettingsOriginRangeRequests>,
    #[serde(rename = "read_timeout", default, skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<RulesetsSetCacheSettingsReadTimeout>,
    #[serde(
        rename = "respect_strong_etags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub respect_strong_etags: Option<RulesetsSetCacheSettingsRespectStrongEtags>,
    #[serde(rename = "serve_stale", default, skip_serializing_if = "Option::is_none")]
    pub serve_stale: Option<RulesetsSetCacheSettingsServeStale>,
    #[serde(
        rename = "shared_dictionary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_dictionary: Option<RulesetsSetCacheSettingsSharedDictionary>,
    #[serde(rename = "strip_etags", default, skip_serializing_if = "Option::is_none")]
    pub strip_etags: Option<RulesetsSetCacheSettingsStripETags>,
    #[serde(
        rename = "strip_last_modified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strip_last_modified: Option<RulesetsSetCacheSettingsStripLastModified>,
    #[serde(
        rename = "strip_set_cookie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strip_set_cookie: Option<RulesetsSetCacheSettingsStripSetCookie>,
    #[serde(rename = "vary", default, skip_serializing_if = "Option::is_none")]
    pub vary: Option<RulesetsSetCacheSettingsVary>,
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
pub struct RulesetsResponseRuleVariant17 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant17ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant18ActionParametersVariant1 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
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
pub struct RulesetsResponseRuleVariant18ActionParametersVariant2 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsResponseRuleVariant18ActionParametersVariant3 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
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
pub struct RulesetsResponseRuleVariant18ActionParametersVariant4 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub struct RulesetsResponseRuleVariant18ActionParametersVariant5 {
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "values")]
    pub values: RulesetsSetCacheTagsValues,
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
pub struct RulesetsResponseRuleVariant18ActionParametersVariant6 {
    #[serde(rename = "expression")]
    pub expression: RulesetsSetCacheTagsExpression,
    #[serde(rename = "operation")]
    pub operation: String,
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
pub enum RulesetsResponseRuleVariant18ActionParameters {
    Variant1(RulesetsResponseRuleVariant18ActionParametersVariant1),
    Variant2(RulesetsResponseRuleVariant18ActionParametersVariant2),
    Variant3(RulesetsResponseRuleVariant18ActionParametersVariant3),
    Variant4(RulesetsResponseRuleVariant18ActionParametersVariant4),
    Variant5(RulesetsResponseRuleVariant18ActionParametersVariant5),
    Variant6(RulesetsResponseRuleVariant18ActionParametersVariant6),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant18 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant18ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant19ActionParameters {
    ///Whether to enable Automatic HTTPS Rewrites.
    #[serde(
        rename = "automatic_https_rewrites",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_https_rewrites: Option<bool>,
    #[serde(rename = "autominify", default, skip_serializing_if = "Option::is_none")]
    pub autominify: Option<RulesetsSetConfigAutominify>,
    ///Whether to enable Browser Integrity Check (BIC).
    #[serde(rename = "bic", default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<bool>,
    ///Whether to enable content conversion (e.g., HTML to Markdown).
    #[serde(
        rename = "content_converter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_converter: Option<bool>,
    ///Whether to disable Cloudflare Apps.
    #[serde(rename = "disable_apps", default, skip_serializing_if = "Option::is_none")]
    pub disable_apps: Option<bool>,
    ///Whether to disable Pay Per Crawl.
    #[serde(
        rename = "disable_pay_per_crawl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_pay_per_crawl: Option<bool>,
    ///Whether to disable Real User Monitoring (RUM).
    #[serde(rename = "disable_rum", default, skip_serializing_if = "Option::is_none")]
    pub disable_rum: Option<bool>,
    ///Whether to disable Zaraz.
    #[serde(rename = "disable_zaraz", default, skip_serializing_if = "Option::is_none")]
    pub disable_zaraz: Option<bool>,
    ///Whether to enable Email Obfuscation.
    #[serde(
        rename = "email_obfuscation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_obfuscation: Option<bool>,
    ///Whether to enable Cloudflare Fonts.
    #[serde(rename = "fonts", default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<bool>,
    ///Whether to enable Hotlink Protection.
    #[serde(
        rename = "hotlink_protection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hotlink_protection: Option<bool>,
    ///Whether to enable Mirage.
    #[serde(rename = "mirage", default, skip_serializing_if = "Option::is_none")]
    pub mirage: Option<bool>,
    ///Whether to enable Opportunistic Encryption.
    #[serde(
        rename = "opportunistic_encryption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub opportunistic_encryption: Option<bool>,
    ///The Polish level to configure.
    #[serde(rename = "polish", default, skip_serializing_if = "Option::is_none")]
    pub polish: Option<String>,
    ///Whether to redirect verified AI training crawlers to canonical URLs found in the HTML response.
    #[serde(
        rename = "redirects_for_ai_training",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redirects_for_ai_training: Option<bool>,
    ///The request body buffering mode.
    #[serde(
        rename = "request_body_buffering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_body_buffering: Option<String>,
    ///The response body buffering mode.
    #[serde(
        rename = "response_body_buffering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_body_buffering: Option<String>,
    ///Whether to enable Rocket Loader.
    #[serde(rename = "rocket_loader", default, skip_serializing_if = "Option::is_none")]
    pub rocket_loader: Option<bool>,
    ///The Security Level to configure.
    #[serde(rename = "security_level", default, skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,
    ///Whether to enable Server-Side Excludes.
    #[serde(
        rename = "server_side_excludes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_excludes: Option<bool>,
    ///The SSL level to configure.
    #[serde(rename = "ssl", default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,
    ///Whether to enable Signed Exchanges (SXG).
    #[serde(rename = "sxg", default, skip_serializing_if = "Option::is_none")]
    pub sxg: Option<bool>,
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
pub struct RulesetsResponseRuleVariant19 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant19ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant20ActionParameters {
    #[serde(rename = "phase", default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<RulesetsSkipPhase>,
    #[serde(rename = "phases", default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<RulesetsSkipPhases>,
    #[serde(rename = "products", default, skip_serializing_if = "Option::is_none")]
    pub products: Option<RulesetsSkipProducts>,
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RulesetsSkipRules>,
    #[serde(rename = "ruleset", default, skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<RulesetsSkipRuleset>,
    #[serde(rename = "rulesets", default, skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<RulesetsSkipRulesets>,
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
pub struct RulesetsResponseRuleVariant20 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant20ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant21ActionParametersLinkMaze {
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
pub struct RulesetsResponseRuleVariant21ActionParameters {
    ///Enables the link maze transformation on the response.
    #[serde(rename = "link_maze")]
    pub link_maze: RulesetsResponseRuleVariant21ActionParametersLinkMaze,
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
pub struct RulesetsResponseRuleVariant21 {
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(
        rename = "action_parameters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub action_parameters: Option<RulesetsResponseRuleVariant21ActionParameters>,
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "exposed_credential_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_credential_check: Option<RulesetsRuleExposedCredentialCheck>,
    ///The expression defining which traffic will match the rule.
    #[serde(rename = "expression", default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<RulesetsRuleLogging>,
    #[serde(rename = "ratelimit", default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    ///The version of the rule.
    #[serde(rename = "version")]
    pub version: String,
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
pub enum RulesetsResponseRule {
    Variant1(RulesetsResponseRuleVariant1),
    Variant2(RulesetsResponseRuleVariant2),
    Variant3(RulesetsResponseRuleVariant3),
    Variant4(RulesetsResponseRuleVariant4),
    Variant5(RulesetsResponseRuleVariant5),
    Variant6(RulesetsResponseRuleVariant6),
    Variant7(RulesetsResponseRuleVariant7),
    Variant8(RulesetsResponseRuleVariant8),
    Variant9(RulesetsResponseRuleVariant9),
    Variant10(RulesetsResponseRuleVariant10),
    Variant11(RulesetsResponseRuleVariant11),
    Variant12(RulesetsResponseRuleVariant12),
    Variant13(RulesetsResponseRuleVariant13),
    Variant14(RulesetsResponseRuleVariant14),
    Variant15(RulesetsResponseRuleVariant15),
    Variant16(RulesetsResponseRuleVariant16),
    Variant17(RulesetsResponseRuleVariant17),
    Variant18(RulesetsResponseRuleVariant18),
    Variant19(RulesetsResponseRuleVariant19),
    Variant20(RulesetsResponseRuleVariant20),
    Variant21(RulesetsResponseRuleVariant21),
}
pub type RulesetsResponseRules = Vec<RulesetsResponseRule>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct CreateZoneRulesetResponseResultVariant1 {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase")]
    pub phase: RulesetsRulesetPhase,
    #[serde(rename = "rules")]
    pub rules: RulesetsResponseRules,
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct CreateZoneRulesetResponseResultVariant2 {
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
pub enum CreateZoneRulesetResponseResult {
    Variant1(CreateZoneRulesetResponseResultVariant1),
    Variant2(CreateZoneRulesetResponseResultVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct CreateZoneRulesetResponse {
    #[serde(rename = "errors")]
    pub errors: RulesetsErrors,
    #[serde(rename = "messages")]
    pub messages: RulesetsMessages,
    ///A result.
    #[serde(rename = "result")]
    pub result: CreateZoneRulesetResponseResult,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordPost {
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
pub struct DnsRecordsMessagesItemSource {
    #[serde(rename = "pointer", default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
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
pub struct DnsRecordsMessagesItem {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(
        rename = "documentation_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub documentation_url: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<DnsRecordsMessagesItemSource>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type DnsRecordsMessages = Vec<DnsRecordsMessagesItem>;
pub type DnsRecordsIdentifier = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseMeta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(rename = "dead_glue", default, skip_serializing_if = "Option::is_none")]
    pub dead_glue: Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(rename = "is_glue", default, skip_serializing_if = "Option::is_none")]
    pub is_glue: Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(rename = "shadowed_by", default, skip_serializing_if = "Option::is_none")]
    pub shadowed_by: Option<Vec<String>>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(
        rename = "shadowed_records_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shadowed_records_count: Option<i64>,
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
pub struct DnsRecordsDnsRecordResponse {
    ///When the record comment was last modified. Omitted if there is no comment.
    #[serde(
        rename = "comment_modified_on",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_modified_on: Option<String>,
    ///When the record was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "id")]
    pub id: DnsRecordsIdentifier,
    ///Extra Cloudflare-specific metadata about the record.
    #[serde(rename = "meta")]
    pub meta: DnsRecordsDnsRecordResponseMeta,
    ///When the record was last modified.
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    ///Whether the record can be proxied by Cloudflare or not.
    #[serde(rename = "proxiable")]
    pub proxiable: bool,
    ///When the record tags were last modified. Omitted if there are no tags.
    #[serde(
        rename = "tags_modified_on",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tags_modified_on: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsResponseSingle {
    #[serde(rename = "errors")]
    pub errors: DnsRecordsMessages,
    #[serde(rename = "messages")]
    pub messages: DnsRecordsMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<DnsRecordsDnsRecordResponse>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct DnsRecordsForAZoneDeleteDnsRecordResponseResult {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<DnsRecordsIdentifier>,
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
pub struct DnsRecordsForAZoneDeleteDnsRecordResponse {
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<DnsRecordsForAZoneDeleteDnsRecordResponseResult>,
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
pub struct DnsRecordsDnsRecordPatch {
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
pub struct GetZoneRulesetResponseResult {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase")]
    pub phase: RulesetsRulesetPhase,
    #[serde(rename = "rules")]
    pub rules: RulesetsResponseRules,
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct GetZoneRulesetResponse {
    #[serde(rename = "errors")]
    pub errors: RulesetsErrors,
    #[serde(rename = "messages")]
    pub messages: RulesetsMessages,
    ///A result.
    #[serde(rename = "result")]
    pub result: GetZoneRulesetResponseResult,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateProjectRequestBuildConfig {
    ///Enable build caching for the project.
    #[serde(rename = "build_caching", default, skip_serializing_if = "Option::is_none")]
    pub build_caching: Option<bool>,
    ///Command used to build project.
    #[serde(rename = "build_command", default, skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    ///Output directory of the build.
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewAiBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewAiBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewBrowsersAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewBrowsersAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewD1DatabasesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewD1DatabasesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty,
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
pub enum PagesProjectCreateProjectRequestDeploymentConfigsPreviewEnvVarsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewEnvVars {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewEnvVarsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewKvNamespacesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewKvNamespacesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewLimits {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewPlacement {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewQueueProducersAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewQueueProducersAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewR2BucketsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewR2BucketsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment", default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewServicesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreviewVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsPreview {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewAiBindings,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(
        rename = "always_use_latest_compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_use_latest_compatibility_date: Option<bool>,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewBrowsers,
    >,
    ///The major version of the build image to use for Pages Functions.
    #[serde(
        rename = "build_image_major_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_image_major_version: Option<i64>,
    ///Compatibility date used for Pages Functions.
    #[serde(
        rename = "compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_date: Option<String>,
    ///Compatibility flags used for Pages Functions.
    #[serde(
        rename = "compatibility_flags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_flags: Option<Vec<String>>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewD1Databases,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewDurableObjectNamespaces,
    >,
    ///Environment variables used for builds and Pages Functions.
    #[serde(rename = "env_vars", default, skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewEnvVars,
    >,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open", default, skip_serializing_if = "Option::is_none")]
    pub fail_open: Option<bool>,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectCreateProjectRequestDeploymentConfigsPreviewLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewPlacement,
    >,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewR2Buckets,
    >,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewServices,
    >,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsPreviewVectorizeBindings,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionAiBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionAiBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionBrowsersAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionBrowsersAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionD1DatabasesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionD1DatabasesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty,
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
#[serde(untagged)]
pub enum PagesProjectCreateProjectRequestDeploymentConfigsProductionEnvVarsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionEnvVars {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionEnvVarsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionKvNamespacesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionKvNamespacesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionLimits {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionPlacement {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionQueueProducersAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionQueueProducersAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionR2BucketsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionR2BucketsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment", default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionServicesAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProductionVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectCreateProjectRequestDeploymentConfigsProductionVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigsProduction {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionAiBindings,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(
        rename = "always_use_latest_compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_use_latest_compatibility_date: Option<bool>,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionBrowsers,
    >,
    ///The major version of the build image to use for Pages Functions.
    #[serde(
        rename = "build_image_major_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_image_major_version: Option<i64>,
    ///Compatibility date used for Pages Functions.
    #[serde(
        rename = "compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_date: Option<String>,
    ///Compatibility flags used for Pages Functions.
    #[serde(
        rename = "compatibility_flags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_flags: Option<Vec<String>>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionD1Databases,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionDurableObjectNamespaces,
    >,
    ///Environment variables used for builds and Pages Functions.
    #[serde(rename = "env_vars", default, skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionEnvVars,
    >,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open", default, skip_serializing_if = "Option::is_none")]
    pub fail_open: Option<bool>,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionLimits,
    >,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionPlacement,
    >,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionR2Buckets,
    >,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionServices,
    >,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectCreateProjectRequestDeploymentConfigsProductionVectorizeBindings,
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
pub struct PagesProjectCreateProjectRequestDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(rename = "preview", default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<PagesProjectCreateProjectRequestDeploymentConfigsPreview>,
    ///Configs for production deploys.
    #[serde(rename = "production", default, skip_serializing_if = "Option::is_none")]
    pub production: Option<PagesProjectCreateProjectRequestDeploymentConfigsProduction>,
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
pub struct PagesProjectCreateProjectRequestSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    #[serde(
        rename = "deployments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployments_enabled: Option<bool>,
    ///The owner of the repository.
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    ///The owner ID of the repository.
    #[serde(rename = "owner_id", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_excludes", default, skip_serializing_if = "Option::is_none")]
    pub path_excludes: Option<Vec<String>>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_includes", default, skip_serializing_if = "Option::is_none")]
    pub path_includes: Option<Vec<String>>,
    ///Whether to enable PR comments.
    #[serde(
        rename = "pr_comments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pr_comments_enabled: Option<bool>,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(
        rename = "preview_branch_excludes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_branch_excludes: Option<Vec<String>>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(
        rename = "preview_branch_includes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_branch_includes: Option<Vec<String>>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    #[serde(
        rename = "preview_deployment_setting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_deployment_setting: Option<String>,
    ///The production branch of the repository.
    #[serde(
        rename = "production_branch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_branch: Option<String>,
    ///Whether to trigger a production deployment on commits to the production branch.
    #[serde(
        rename = "production_deployments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_deployments_enabled: Option<bool>,
    ///The ID of the repository.
    #[serde(rename = "repo_id", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    ///The name of the repository.
    #[serde(rename = "repo_name", default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
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
pub struct PagesProjectCreateProjectRequestSource {
    #[serde(rename = "config")]
    pub config: PagesProjectCreateProjectRequestSourceConfig,
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
pub struct PagesProjectCreateProjectRequest {
    ///Configs for the project build process.
    #[serde(rename = "build_config", default, skip_serializing_if = "Option::is_none")]
    pub build_config: Option<PagesProjectCreateProjectRequestBuildConfig>,
    ///Configs for deployments in a project.
    #[serde(
        rename = "deployment_configs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_configs: Option<PagesProjectCreateProjectRequestDeploymentConfigs>,
    ///Name of the project.
    #[serde(rename = "name")]
    pub name: String,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(rename = "production_branch")]
    pub production_branch: String,
    ///Configs for the project source control.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesProjectCreateProjectRequestSource>,
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
pub struct PagesMessagesItemSource {
    #[serde(rename = "pointer", default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
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
pub struct PagesMessagesItem {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(
        rename = "documentation_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub documentation_url: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesMessagesItemSource>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type PagesMessages = Vec<PagesMessagesItem>;
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
pub struct PagesProjectCanonicalDeploymentDeploymentTriggerMetadata {
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
pub struct PagesProjectCanonicalDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectCanonicalDeploymentDeploymentTriggerMetadata,
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
pub struct PagesProjectCanonicalDeployment {
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
    pub deployment_trigger: PagesProjectCanonicalDeploymentDeploymentTrigger,
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
pub struct PagesProjectDeploymentConfigsPreviewAiBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewAiBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewBrowsersAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewBrowsersAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewD1DatabasesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewD1DatabasesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewKvNamespacesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewKvNamespacesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewLimits {
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
pub struct PagesProjectDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
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
pub struct PagesProjectDeploymentConfigsPreviewQueueProducersAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewQueueProducersAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewR2BucketsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewR2BucketsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewServicesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewServicesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsPreviewVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsPreview {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<PagesProjectDeploymentConfigsPreviewAiBindings>,
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
        PagesProjectDeploymentConfigsPreviewAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectDeploymentConfigsPreviewBrowsers>,
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
    pub d1_databases: Option<PagesProjectDeploymentConfigsPreviewD1Databases>,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectDeploymentConfigsPreviewDurableObjectNamespaces,
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
        PagesProjectDeploymentConfigsPreviewHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<PagesProjectDeploymentConfigsPreviewKvNamespaces>,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectDeploymentConfigsPreviewLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<PagesProjectDeploymentConfigsPreviewMtlsCertificates>,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectDeploymentConfigsPreviewPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<PagesProjectDeploymentConfigsPreviewQueueProducers>,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectDeploymentConfigsPreviewR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectDeploymentConfigsPreviewServices>,
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
        PagesProjectDeploymentConfigsPreviewVectorizeBindings,
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
pub struct PagesProjectDeploymentConfigsProductionAiBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionAiBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionBrowsersAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionBrowsersAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionD1DatabasesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionD1DatabasesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionKvNamespacesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionKvNamespacesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionLimits {
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
pub struct PagesProjectDeploymentConfigsProductionMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionPlacement {
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
pub struct PagesProjectDeploymentConfigsProductionQueueProducersAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionQueueProducersAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionR2BucketsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionR2BucketsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionServicesAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionServicesAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProductionVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectDeploymentConfigsProductionVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectDeploymentConfigsProductionVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectDeploymentConfigsProduction {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<PagesProjectDeploymentConfigsProductionAiBindings>,
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
        PagesProjectDeploymentConfigsProductionAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<PagesProjectDeploymentConfigsProductionBrowsers>,
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
    pub d1_databases: Option<PagesProjectDeploymentConfigsProductionD1Databases>,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectDeploymentConfigsProductionDurableObjectNamespaces,
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
        PagesProjectDeploymentConfigsProductionHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<PagesProjectDeploymentConfigsProductionKvNamespaces>,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectDeploymentConfigsProductionLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectDeploymentConfigsProductionMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<PagesProjectDeploymentConfigsProductionPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<PagesProjectDeploymentConfigsProductionQueueProducers>,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<PagesProjectDeploymentConfigsProductionR2Buckets>,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<PagesProjectDeploymentConfigsProductionServices>,
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
        PagesProjectDeploymentConfigsProductionVectorizeBindings,
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
pub struct PagesProjectDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(rename = "preview")]
    pub preview: PagesProjectDeploymentConfigsPreview,
    ///Configs for production deploys.
    #[serde(rename = "production")]
    pub production: PagesProjectDeploymentConfigsProduction,
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
pub struct PagesProjectLatestDeploymentDeploymentTriggerMetadata {
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
pub struct PagesProjectLatestDeploymentDeploymentTrigger {
    ///Additional info about the trigger.
    #[serde(rename = "metadata")]
    pub metadata: PagesProjectLatestDeploymentDeploymentTriggerMetadata,
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
pub struct PagesProjectLatestDeployment {
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
    pub deployment_trigger: PagesProjectLatestDeploymentDeploymentTrigger,
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
pub struct PagesProject {
    #[serde(rename = "build_config", default, skip_serializing_if = "Option::is_none")]
    pub build_config: Option<PagesBuildConfig>,
    #[serde(rename = "canonical_deployment")]
    pub canonical_deployment: PagesProjectCanonicalDeployment,
    ///When the project was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    ///Configs for deployments in a project.
    #[serde(rename = "deployment_configs")]
    pub deployment_configs: PagesProjectDeploymentConfigs,
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
    pub latest_deployment: PagesProjectLatestDeployment,
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
pub struct PagesProjectCreateProjectResponse {
    #[serde(rename = "errors")]
    pub errors: PagesMessages,
    #[serde(rename = "messages")]
    pub messages: PagesMessages,
    #[serde(rename = "result")]
    pub result: PagesProject,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectDeleteProjectResponseResult {
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
pub struct PagesProjectDeleteProjectResponse {
    #[serde(rename = "errors")]
    pub errors: PagesMessages,
    #[serde(rename = "messages")]
    pub messages: PagesMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<PagesProjectDeleteProjectResponseResult>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectGetProjectResponse {
    #[serde(rename = "errors")]
    pub errors: PagesMessages,
    #[serde(rename = "messages")]
    pub messages: PagesMessages,
    #[serde(rename = "result")]
    pub result: PagesProject,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateProjectRequestBuildConfig {
    ///Enable build caching for the project.
    #[serde(rename = "build_caching", default, skip_serializing_if = "Option::is_none")]
    pub build_caching: Option<bool>,
    ///Command used to build project.
    #[serde(rename = "build_command", default, skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    ///Output directory of the build.
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAiBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAiBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewBrowsersAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewBrowsersAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewD1DatabasesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewD1DatabasesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewDurableObjectNamespacesAdditionalProperty,
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
#[serde(untagged)]
pub enum PagesProjectUpdateProjectRequestDeploymentConfigsPreviewEnvVarsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewEnvVars {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewEnvVarsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewKvNamespacesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewKvNamespacesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewLimits {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewPlacement {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewQueueProducersAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewQueueProducersAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewR2BucketsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewR2BucketsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment", default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewServicesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreviewVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsPreview {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAiBindings,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(
        rename = "always_use_latest_compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_use_latest_compatibility_date: Option<bool>,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewBrowsers,
    >,
    ///The major version of the build image to use for Pages Functions.
    #[serde(
        rename = "build_image_major_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_image_major_version: Option<i64>,
    ///Compatibility date used for Pages Functions.
    #[serde(
        rename = "compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_date: Option<String>,
    ///Compatibility flags used for Pages Functions.
    #[serde(
        rename = "compatibility_flags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_flags: Option<Vec<String>>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewD1Databases,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewDurableObjectNamespaces,
    >,
    ///Environment variables used for builds and Pages Functions.
    #[serde(rename = "env_vars", default, skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewEnvVars,
    >,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open", default, skip_serializing_if = "Option::is_none")]
    pub fail_open: Option<bool>,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<PagesProjectUpdateProjectRequestDeploymentConfigsPreviewLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewPlacement,
    >,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewR2Buckets,
    >,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewServices,
    >,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsPreviewVectorizeBindings,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionAiBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionAiBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionAiBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasetsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionBrowsersAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionBrowsers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionBrowsersAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionD1DatabasesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionD1Databases {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionD1DatabasesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionDurableObjectNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionDurableObjectNamespacesAdditionalProperty,
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
#[serde(untagged)]
pub enum PagesProjectUpdateProjectRequestDeploymentConfigsProductionEnvVarsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionEnvVars {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionEnvVarsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionHyperdriveBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionHyperdriveBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionKvNamespacesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionKvNamespaces {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionKvNamespacesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionLimits {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionMtlsCertificatesAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionMtlsCertificates {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionMtlsCertificatesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionPlacement {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionQueueProducersAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionQueueProducers {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionQueueProducersAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionR2BucketsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionR2Buckets {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionR2BucketsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionServicesAdditionalProperty {
    ///The entrypoint to bind to.
    #[serde(rename = "entrypoint", default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    ///The Service environment.
    #[serde(rename = "environment", default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionServices {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionServicesAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionVectorizeBindingsAdditionalProperty {
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProductionVectorizeBindings {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<
        String,
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionVectorizeBindingsAdditionalProperty,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigsProduction {
    ///Constellation bindings used for Pages Functions.
    #[serde(rename = "ai_bindings", default, skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionAiBindings,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(
        rename = "always_use_latest_compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub always_use_latest_compatibility_date: Option<bool>,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(
        rename = "analytics_engine_datasets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_engine_datasets: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionAnalyticsEngineDatasets,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(rename = "browsers", default, skip_serializing_if = "Option::is_none")]
    pub browsers: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionBrowsers,
    >,
    ///The major version of the build image to use for Pages Functions.
    #[serde(
        rename = "build_image_major_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_image_major_version: Option<i64>,
    ///Compatibility date used for Pages Functions.
    #[serde(
        rename = "compatibility_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_date: Option<String>,
    ///Compatibility flags used for Pages Functions.
    #[serde(
        rename = "compatibility_flags",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_flags: Option<Vec<String>>,
    ///D1 databases used for Pages Functions.
    #[serde(rename = "d1_databases", default, skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionD1Databases,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(
        rename = "durable_object_namespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub durable_object_namespaces: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionDurableObjectNamespaces,
    >,
    ///Environment variables used for builds and Pages Functions.
    #[serde(rename = "env_vars", default, skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionEnvVars,
    >,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(rename = "fail_open", default, skip_serializing_if = "Option::is_none")]
    pub fail_open: Option<bool>,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(
        rename = "hyperdrive_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hyperdrive_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionHyperdriveBindings,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(rename = "kv_namespaces", default, skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionKvNamespaces,
    >,
    ///Limits for Pages Functions.
    #[serde(rename = "limits", default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionLimits,
    >,
    ///mTLS bindings used for Pages Functions.
    #[serde(
        rename = "mtls_certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mtls_certificates: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionMtlsCertificates,
    >,
    ///Placement setting used for Pages Functions.
    #[serde(rename = "placement", default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionPlacement,
    >,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(
        rename = "queue_producers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub queue_producers: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionQueueProducers,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(rename = "r2_buckets", default, skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionR2Buckets,
    >,
    ///Services used for Pages Functions.
    #[serde(rename = "services", default, skip_serializing_if = "Option::is_none")]
    pub services: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionServices,
    >,
    ///The usage model for Pages Functions.
    #[serde(rename = "usage_model", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
    ///Vectorize bindings used for Pages Functions.
    #[serde(
        rename = "vectorize_bindings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vectorize_bindings: Option<
        PagesProjectUpdateProjectRequestDeploymentConfigsProductionVectorizeBindings,
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
pub struct PagesProjectUpdateProjectRequestDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(rename = "preview", default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<PagesProjectUpdateProjectRequestDeploymentConfigsPreview>,
    ///Configs for production deploys.
    #[serde(rename = "production", default, skip_serializing_if = "Option::is_none")]
    pub production: Option<PagesProjectUpdateProjectRequestDeploymentConfigsProduction>,
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
pub struct PagesProjectUpdateProjectRequestSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    #[serde(
        rename = "deployments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployments_enabled: Option<bool>,
    ///The owner of the repository.
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    ///The owner ID of the repository.
    #[serde(rename = "owner_id", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_excludes", default, skip_serializing_if = "Option::is_none")]
    pub path_excludes: Option<Vec<String>>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(rename = "path_includes", default, skip_serializing_if = "Option::is_none")]
    pub path_includes: Option<Vec<String>>,
    ///Whether to enable PR comments.
    #[serde(
        rename = "pr_comments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pr_comments_enabled: Option<bool>,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(
        rename = "preview_branch_excludes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_branch_excludes: Option<Vec<String>>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(
        rename = "preview_branch_includes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_branch_includes: Option<Vec<String>>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    #[serde(
        rename = "preview_deployment_setting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_deployment_setting: Option<String>,
    ///The production branch of the repository.
    #[serde(
        rename = "production_branch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_branch: Option<String>,
    ///Whether to trigger a production deployment on commits to the production branch.
    #[serde(
        rename = "production_deployments_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_deployments_enabled: Option<bool>,
    ///The ID of the repository.
    #[serde(rename = "repo_id", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    ///The name of the repository.
    #[serde(rename = "repo_name", default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
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
pub struct PagesProjectUpdateProjectRequestSource {
    #[serde(rename = "config")]
    pub config: PagesProjectUpdateProjectRequestSourceConfig,
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
pub struct PagesProjectUpdateProjectRequest {
    ///Configs for the project build process.
    #[serde(rename = "build_config", default, skip_serializing_if = "Option::is_none")]
    pub build_config: Option<PagesProjectUpdateProjectRequestBuildConfig>,
    ///Configs for deployments in a project.
    #[serde(
        rename = "deployment_configs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_configs: Option<PagesProjectUpdateProjectRequestDeploymentConfigs>,
    ///Name of the project.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(
        rename = "production_branch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_branch: Option<String>,
    ///Configs for the project source control.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesProjectUpdateProjectRequestSource>,
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
pub struct PagesProjectUpdateProjectResponse {
    #[serde(rename = "errors")]
    pub errors: PagesMessages,
    #[serde(rename = "messages")]
    pub messages: PagesMessages,
    #[serde(rename = "result")]
    pub result: PagesProject,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct UpdateZoneRulesetRequest {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<RulesetsRulesetKind>,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase", default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<RulesetsRulesetPhase>,
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RulesetsRequestRules>,
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct UpdateZoneRulesetResponseResultVariant1 {
    ///An informative description of the ruleset.
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "kind")]
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    ///The human-readable name of the ruleset.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase")]
    pub phase: RulesetsRulesetPhase,
    #[serde(rename = "rules")]
    pub rules: RulesetsResponseRules,
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct UpdateZoneRulesetResponseResultVariant2 {
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
pub enum UpdateZoneRulesetResponseResult {
    Variant1(UpdateZoneRulesetResponseResultVariant1),
    Variant2(UpdateZoneRulesetResponseResultVariant2),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct UpdateZoneRulesetResponse {
    #[serde(rename = "errors")]
    pub errors: RulesetsErrors,
    #[serde(rename = "messages")]
    pub messages: RulesetsMessages,
    ///A result.
    #[serde(rename = "result")]
    pub result: UpdateZoneRulesetResponseResult,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesMessagesItem {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesMessages = Vec<ZonesMessagesItem>;
pub type ZonesIdentifier = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseSingleIdResult {
    #[serde(rename = "id")]
    pub id: ZonesIdentifier,
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
pub struct ZonesApiResponseSingleId {
    #[serde(rename = "errors")]
    pub errors: ZonesMessages,
    #[serde(rename = "messages")]
    pub messages: ZonesMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ZonesApiResponseSingleIdResult>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesZoneAccount {
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
pub struct ZonesZoneMeta {
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
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesZoneOwner {
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
pub struct ZonesZonePlan {
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
pub struct ZonesZoneTenant {
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
pub struct ZonesZoneTenantUnit {
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
pub struct ZonesZone {
    ///The account the zone belongs to.
    #[serde(rename = "account")]
    pub account: ZonesZoneAccount,
    /**The last time proof of ownership was detected and the zone was made
active.*/
    #[serde(rename = "activated_on", default, skip_serializing_if = "Option::is_none")]
    pub activated_on: Option<String>,
    /**Allows the customer to use a custom apex.
*Tenants Only Configuration*.*/
    #[serde(rename = "cname_suffix", default, skip_serializing_if = "Option::is_none")]
    pub cname_suffix: Option<String>,
    ///When the zone was created.
    #[serde(rename = "created_on")]
    pub created_on: String,
    /**The interval (in seconds) from when development mode expires
(positive integer) or last expired (negative integer) for the
domain. If development mode has never been enabled, this value is 0.*/
    #[serde(rename = "development_mode")]
    pub development_mode: f64,
    ///Identifier
    #[serde(rename = "id")]
    pub id: String,
    ///Metadata about the zone.
    #[serde(rename = "meta")]
    pub meta: ZonesZoneMeta,
    ///When the zone was last modified.
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    ///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
    #[serde(rename = "name")]
    pub name: String,
    ///The name servers Cloudflare assigns to a zone.
    #[serde(rename = "name_servers")]
    pub name_servers: Vec<String>,
    ///DNS host at the time of switching to Cloudflare.
    #[serde(
        rename = "original_dnshost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_dnshost: Option<String>,
    ///Original name servers before moving to Cloudflare.
    #[serde(
        rename = "original_name_servers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_name_servers: Option<Vec<String>>,
    ///Registrar for the domain at the time of switching to Cloudflare.
    #[serde(
        rename = "original_registrar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_registrar: Option<String>,
    ///The owner of the zone.
    #[serde(rename = "owner")]
    pub owner: ZonesZoneOwner,
    #[serde(rename = "paused", default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<ZonesPaused>,
    ///Legacy permissions based on legacy user membership information.
    #[serde(rename = "permissions", default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    ///A Zones subscription information.
    #[serde(rename = "plan")]
    pub plan: ZonesZonePlan,
    ///The zone status on Cloudflare.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///The root organizational unit that this zone belongs to (such as a tenant or organization).
    #[serde(rename = "tenant", default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<ZonesZoneTenant>,
    ///The immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization).
    #[serde(rename = "tenant_unit", default, skip_serializing_if = "Option::is_none")]
    pub tenant_unit: Option<ZonesZoneTenantUnit>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ZonesType>,
    ///An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    #[serde(
        rename = "vanity_name_servers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanity_name_servers: Option<Vec<String>>,
    ///Verification key for partial zone setup.
    #[serde(
        rename = "verification_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_key: Option<String>,
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
pub struct Zones0GetResponse {
    #[serde(rename = "errors")]
    pub errors: ZonesMessages,
    #[serde(rename = "messages")]
    pub messages: ZonesMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ZonesZone>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct Zones0PatchRequestPlan {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesVanityNameServers = Vec<String>;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct Zones0PatchRequest {
    #[serde(rename = "paused", default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<ZonesPaused>,
    /**(Deprecated) Please use the `/zones/{zone_id}/subscription` API
to update a zone's plan. Changing this value will create/cancel
associated subscriptions. To view available plans for this zone,
see Zone Plans.
*/
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Zones0PatchRequestPlan>,
    /**A full zone implies that DNS is hosted with Cloudflare. A partial
zone is typically a partner-hosted zone or a CNAME setup. This
parameter is only available to Enterprise customers or if it has
been explicitly enabled on a zone.
*/
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        rename = "vanity_name_servers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanity_name_servers: Option<ZonesVanityNameServers>,
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
pub struct Zones0PatchResponse {
    #[serde(rename = "errors")]
    pub errors: ZonesMessages,
    #[serde(rename = "messages")]
    pub messages: ZonesMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ZonesZone>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesPostRequestAccount {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ZonesIdentifier>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
pub type ZonesName = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct ZonesPostRequest {
    #[serde(rename = "account")]
    pub account: ZonesPostRequestAccount,
    #[serde(rename = "name")]
    pub name: ZonesName,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ZonesType>,
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
pub struct ZonesPostResponse {
    #[serde(rename = "errors")]
    pub errors: ZonesMessages,
    #[serde(rename = "messages")]
    pub messages: ZonesMessages,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ZonesZone>,
    ///Whether the API call was successful.
    #[serde(rename = "success")]
    pub success: bool,
}
#[derive(Clone)]
pub struct ProviderClient {
    http: reqwest::Client,
    base_url: String,
}
impl ProviderClient {
    pub fn new(http: reqwest::Client, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into(),
        }
    }
    pub fn account_creation(
        &self,
        body: &IamCreateAccount,
    ) -> core::ApiRequest<IamResponseSingleAccount> {
        let mut path = "/accounts".to_owned();
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("post".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn account_deletion(
        &self,
        account_id: &str,
    ) -> core::ApiRequest<IamApiResponseSingleId> {
        let mut path = "/accounts/{account_id}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("delete".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn accounts_account_details(
        &self,
        account_id: &str,
    ) -> core::ApiRequest<IamResponseSingleAccount> {
        let mut path = "/accounts/{account_id}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("get".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn accounts_update_account(
        &self,
        account_id: &str,
        body: &IamComponentsSchemasAccount,
    ) -> core::ApiRequest<IamResponseSingleAccount> {
        let mut path = "/accounts/{account_id}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("put".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn create_zone_ruleset(
        &self,
        zone_id: &str,
        body: &CreateZoneRulesetRequest,
    ) -> core::ApiRequest<CreateZoneRulesetResponse> {
        let mut path = "/zones/{zone_id}/rulesets".to_owned();
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("post".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn delete_zone_ruleset(
        &self,
        ruleset_id: &str,
        zone_id: &str,
    ) -> core::ApiRequest<()> {
        let mut path = "/zones/{zone_id}/rulesets/{ruleset_id}".to_owned();
        path = path.replace("{ruleset_id}", urlencoding::encode(ruleset_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("delete".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn dns_records_for_a_zone_create_dns_record(
        &self,
        zone_id: &str,
        body: &DnsRecordsDnsRecordPost,
    ) -> core::ApiRequest<DnsRecordsDnsResponseSingle> {
        let mut path = "/zones/{zone_id}/dns_records".to_owned();
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("post".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn dns_records_for_a_zone_delete_dns_record(
        &self,
        dns_record_id: &str,
        zone_id: &str,
    ) -> core::ApiRequest<DnsRecordsForAZoneDeleteDnsRecordResponse> {
        let mut path = "/zones/{zone_id}/dns_records/{dns_record_id}".to_owned();
        path = path
            .replace("{dns_record_id}", urlencoding::encode(dns_record_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("delete".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn dns_records_for_a_zone_dns_record_details(
        &self,
        dns_record_id: &str,
        zone_id: &str,
    ) -> core::ApiRequest<DnsRecordsDnsResponseSingle> {
        let mut path = "/zones/{zone_id}/dns_records/{dns_record_id}".to_owned();
        path = path
            .replace("{dns_record_id}", urlencoding::encode(dns_record_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("get".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn dns_records_for_a_zone_patch_dns_record(
        &self,
        dns_record_id: &str,
        zone_id: &str,
        body: &DnsRecordsDnsRecordPatch,
    ) -> core::ApiRequest<DnsRecordsDnsResponseSingle> {
        let mut path = "/zones/{zone_id}/dns_records/{dns_record_id}".to_owned();
        path = path
            .replace("{dns_record_id}", urlencoding::encode(dns_record_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("patch".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn get_zone_ruleset(
        &self,
        ruleset_id: &str,
        zone_id: &str,
    ) -> core::ApiRequest<GetZoneRulesetResponse> {
        let mut path = "/zones/{zone_id}/rulesets/{ruleset_id}".to_owned();
        path = path.replace("{ruleset_id}", urlencoding::encode(ruleset_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("get".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn pages_project_create_project(
        &self,
        account_id: &str,
        body: &PagesProjectCreateProjectRequest,
    ) -> core::ApiRequest<PagesProjectCreateProjectResponse> {
        let mut path = "/accounts/{account_id}/pages/projects".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("post".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn pages_project_delete_project(
        &self,
        account_id: &str,
        project_name: &str,
    ) -> core::ApiRequest<PagesProjectDeleteProjectResponse> {
        let mut path = "/accounts/{account_id}/pages/projects/{project_name}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        path = path
            .replace("{project_name}", urlencoding::encode(project_name).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("delete".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn pages_project_get_project(
        &self,
        account_id: &str,
        project_name: &str,
    ) -> core::ApiRequest<PagesProjectGetProjectResponse> {
        let mut path = "/accounts/{account_id}/pages/projects/{project_name}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        path = path
            .replace("{project_name}", urlencoding::encode(project_name).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("get".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn pages_project_update_project(
        &self,
        account_id: &str,
        project_name: &str,
        body: &PagesProjectUpdateProjectRequest,
    ) -> core::ApiRequest<PagesProjectUpdateProjectResponse> {
        let mut path = "/accounts/{account_id}/pages/projects/{project_name}".to_owned();
        path = path.replace("{account_id}", urlencoding::encode(account_id).as_ref());
        path = path
            .replace("{project_name}", urlencoding::encode(project_name).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("patch".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn update_zone_ruleset(
        &self,
        ruleset_id: &str,
        zone_id: &str,
        body: &UpdateZoneRulesetRequest,
    ) -> core::ApiRequest<UpdateZoneRulesetResponse> {
        let mut path = "/zones/{zone_id}/rulesets/{ruleset_id}".to_owned();
        path = path.replace("{ruleset_id}", urlencoding::encode(ruleset_id).as_ref());
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("put".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn zones_0_delete(
        &self,
        zone_id: &str,
    ) -> core::ApiRequest<ZonesApiResponseSingleId> {
        let mut path = "/zones/{zone_id}".to_owned();
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("delete".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn zones_0_get(&self, zone_id: &str) -> core::ApiRequest<Zones0GetResponse> {
        let mut path = "/zones/{zone_id}".to_owned();
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("get".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        core::ApiRequest::new(request)
    }
    pub fn zones_0_patch(
        &self,
        zone_id: &str,
        body: &Zones0PatchRequest,
    ) -> core::ApiRequest<Zones0PatchResponse> {
        let mut path = "/zones/{zone_id}".to_owned();
        path = path.replace("{zone_id}", urlencoding::encode(zone_id).as_ref());
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("patch".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
    pub fn zones_post(
        &self,
        body: &ZonesPostRequest,
    ) -> core::ApiRequest<ZonesPostResponse> {
        let mut path = "/zones".to_owned();
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path,);
        let method = reqwest::Method::from_bytes("post".as_bytes())
            .expect("validated HTTP method");
        let mut request = self.http.request(method, url);
        request = request.json(body);
        core::ApiRequest::new(request)
    }
}
