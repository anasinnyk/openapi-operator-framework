use crate::api::CredentialError;
use k8s_openapi::api::core::v1::{Secret, SecretKeySelector};
use kube::Api;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

pub type SecretReference = SecretKeySelector;

pub async fn resolve_secret_key(
    client: &kube::Client,
    namespace: &str,
    selector: &SecretKeySelector,
) -> Result<String, CredentialError> {
    let secret_name = selector.name.clone();

    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);

    let secret = secrets.get(&secret_name).await?;

    if let Some(value) = secret
        .string_data
        .as_ref()
        .and_then(|data| data.get(&selector.key))
    {
        return Ok(value.clone());
    }

    let bytes = secret
        .data
        .as_ref()
        .and_then(|data| data.get(&selector.key))
        .ok_or_else(|| {
            CredentialError::MissingValue(format!(
                "Secret {secret_name:?} does not contain key {:?}",
                selector.key,
            ))
        })?
        .0
        .clone();

    String::from_utf8(bytes).map_err(|source| CredentialError::InvalidSecretUtf8 {
        secret: secret_name.clone(),
        key: selector.key.clone(),
        source,
    })
}
