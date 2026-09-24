use crate::error::{ClientError, CredentialError};

pub trait ApiCredential {
    fn apply(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

pub struct ApiRequest<T> {
    pub request: reqwest::RequestBuilder,
    marker: std::marker::PhantomData<T>,
}

impl<T> ApiRequest<T> {
    pub fn new(request: reqwest::RequestBuilder) -> Self {
        Self {
            request,
            marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn with_credentials(mut self, credentials: &impl ApiCredential) -> Self {
        self.request = credentials.apply(self.request);
        self
    }

    pub fn into_request(self) -> reqwest::RequestBuilder {
        self.request
    }
}

impl<T> ApiRequest<T>
where
    T: serde::de::DeserializeOwned,
{
    pub async fn send(self) -> Result<T, ClientError> {
        let response = self.request.send().await?.error_for_status()?;

        let body = response.bytes().await?;

        let body = if body.is_empty() {
            &b"null"[..]
        } else {
            body.as_ref()
        };

        Ok(serde_json::from_slice(body)?)
    }

    pub async fn send_optional(self) -> Result<Option<T>, ClientError> {
        let response = self.request.send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = response.error_for_status()?;
        let body = response.bytes().await?;

        let body = if body.is_empty() {
            &b"null"[..]
        } else {
            body.as_ref()
        };

        Ok(Some(serde_json::from_slice(body)?))
    }
}

pub struct HeaderCredential {
    name: reqwest::header::HeaderName,
    value: reqwest::header::HeaderValue,
}

impl HeaderCredential {
    pub fn new(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self, CredentialError> {
        let name = reqwest::header::HeaderName::from_bytes(name.as_ref().as_bytes())?;

        let mut value = reqwest::header::HeaderValue::from_str(value.as_ref())?;

        value.set_sensitive(true);

        Ok(Self { name, value })
    }
}

impl ApiCredential for HeaderCredential {
    fn apply(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.header(self.name.clone(), self.value.clone())
    }
}

pub struct BearerCredential(HeaderCredential);

impl BearerCredential {
    pub fn new(token: impl AsRef<str>) -> Result<Self, CredentialError> {
        Ok(Self(HeaderCredential::new(
            reqwest::header::AUTHORIZATION.as_str(),
            format!("Bearer {}", token.as_ref()),
        )?))
    }
}

impl ApiCredential for BearerCredential {
    fn apply(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        self.0.apply(request)
    }
}
