use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("unknown schema: {0}")]
    UnknownSchema(String),

    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),

    #[error("invalid Rust identifier: {0}")]
    InvalidIdentifier(String),

    #[error(transparent)]
    Syntax(#[from] syn::Error),

    #[error("invalid Kubernetes apiVersion: {0}")]
    InvalidApiVersion(String),
}
