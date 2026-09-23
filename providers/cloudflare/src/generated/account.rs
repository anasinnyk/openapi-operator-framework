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
    #[serde(rename = "apiTokenSecretRef")]
    pub api_token_secret_ref: core::reference::SecretReference,
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
pub async fn observe(
    kube_client: &kube::Client,
    provider_client: &crate::generated::client::ProviderClient,
    resource: &Account,
) -> Result<core::reconciler::Observation, core::error::ReconcileError> {
    let account_id = (async {
        core::api::resolve_field_value(&resource, "status.atProvider.id")
    })
        .await?
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing(
                format!("could not resolve path parameter {}", "account_id",),
            )
        })?;
    let request = provider_client.accounts_account_details(&account_id);
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
    resource: &Account,
) -> Result<crate::generated::client::ApiTokenCredential, core::api::CredentialError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            core::api::CredentialError::MissingValue(
                format!("{} has no namespace", "Account",),
            )
        })?;
    let selector = (resource.spec.for_provider.api_token_secret_ref)
        .as_ref()
        .ok_or_else(|| {
            core::api::CredentialError::MissingValue(
                format!(
                    "credential field {} is not set",
                    "spec.forProvider.apiTokenSecretRef",
                ),
            )
        })?;
    let value = core::reference::resolve_secret_key(client, &namespace, selector).await?;
    Ok(crate::generated::client::ApiTokenCredential::new(value)?)
}
pub async fn update_status(
    client: &kube::Client,
    resource: &Account,
    observation: &core::reconciler::Observation,
) -> Result<(), core::error::ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource)
        .ok_or_else(|| {
            core::error::ResolveValueError::Missing("resource has no namespace".into())
        })?;
    let name = kube::ResourceExt::name_any(resource);
    let at_provider: Option<AccountAtProvider> = observation
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
    let status = AccountStatus {
        at_provider,
        observed_generation: resource.metadata.generation,
        conditions: vec![condition],
    };
    let patch = serde_json::json!({ "status" : status, });
    let api: kube::Api<Account> = kube::Api::namespaced(client.clone(), &namespace);
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
    resource: std::sync::Arc<Account>,
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
    _resource: std::sync::Arc<Account>,
    _error: &core::error::ReconcileError,
    _context: std::sync::Arc<
        core::reconciler::ControllerContext<crate::generated::client::ProviderClient>,
    >,
) -> kube::runtime::controller::Action {
    kube::runtime::controller::Action::requeue(std::time::Duration::from_secs(30))
}
