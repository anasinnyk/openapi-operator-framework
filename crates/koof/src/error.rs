#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("invalid credential header name: {0}")]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),

    #[error("invalid credential header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),

    #[error("{0}")]
    MissingValue(String),

    #[error("Secret {secret:?} key {key:?} is not valid UTF-8")]
    InvalidSecretUtf8 {
        secret: String,
        key: String,
        #[source]
        source: std::string::FromUtf8Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveValueError {
    #[error("failed to serialize resource: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Missing(String),

    #[error("field {path:?} is not a scalar value")]
    NonScalar { path: String },

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReconcileError {
    #[error(transparent)]
    Credentials(#[from] CredentialError),

    #[error(transparent)]
    ResolveValue(#[from] ResolveValueError),

    #[error(transparent)]
    Client(#[from] ClientError),

    #[error("failed to serialize provider response: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed: {0}")]
    Kubernetes(#[from] kube::Error),
}
