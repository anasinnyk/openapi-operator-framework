use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use thiserror::Error;

use crate::{
    cloudflare::generated::zone::{self, types},
    resources::ZoneType,
};

const CLOUDFLARE_API_BASE_URL: &str = "https://api.cloudflare.com/client/v4";

#[derive(Clone)]
pub struct CloudflareClientFactory {
    base_url: String,
}

impl Default for CloudflareClientFactory {
    fn default() -> Self {
        Self {
            base_url: CLOUDFLARE_API_BASE_URL.to_owned(),
        }
    }
}

impl CloudflareClientFactory {
    pub fn for_api_token(&self, api_token: &str) -> Result<CloudflareClient, CloudflareError> {
        let mut headers = HeaderMap::new();
        let mut authorization = HeaderValue::from_str(&format!("Bearer {api_token}"))?;
        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(CloudflareClient {
            inner: zone::Client::new_with_client(&self.base_url, http),
        })
    }
}

#[derive(Clone)]
pub struct CloudflareClient {
    inner: zone::Client,
}

impl CloudflareClient {
    pub async fn create_zone(
        &self,
        account_id: &str,
        name: &str,
        zone_type: Option<ZoneType>,
    ) -> Result<types::ZonesZone, CloudflareError> {
        let body = types::ZonesCreateBody {
            account: types::ZonesCreateBodyAccount {
                id: Some(account_id.try_into()?),
            },
            name: name.try_into()?,
            type_: zone_type.map(Into::into),
        };

        let response = self.inner.zones_create(&body).await?;
        response
            .into_inner()
            .result
            .ok_or(CloudflareError::MissingResult("zones_create"))
    }

    pub async fn read_zone(&self, zone_id: &str) -> Result<types::ZonesZone, CloudflareError> {
        let zone_id = zone_id.try_into()?;
        let response = self.inner.zones_read(&zone_id).await?;
        response
            .into_inner()
            .result
            .ok_or(CloudflareError::MissingResult("zones_read"))
    }

    pub async fn update_zone_type(
        &self,
        zone_id: &str,
        zone_type: ZoneType,
    ) -> Result<types::ZonesZone, CloudflareError> {
        let zone_id = zone_id.try_into()?;
        let body = types::ZonesUpdateBody {
            type_: Some(zone_type.into()),
            ..types::ZonesUpdateBody::default()
        };

        let response = self.inner.zones_update(&zone_id, &body).await?;
        response
            .into_inner()
            .result
            .ok_or(CloudflareError::MissingResult("zones_update"))
    }

    pub async fn delete_zone(&self, zone_id: &str) -> Result<(), CloudflareError> {
        let zone_id = zone_id.try_into()?;
        self.inner.zones_delete(&zone_id).await?;
        Ok(())
    }
}

impl From<ZoneType> for types::ZonesType {
    fn from(value: ZoneType) -> Self {
        match value {
            ZoneType::Full => Self::Full,
            ZoneType::Partial => Self::Partial,
        }
    }
}

impl From<ZoneType> for types::ZonesUpdateBodyType {
    fn from(value: ZoneType) -> Self {
        match value {
            ZoneType::Full => Self::Full,
            ZoneType::Partial => Self::Partial,
        }
    }
}

#[derive(Debug, Error)]
pub enum CloudflareError {
    #[error(transparent)]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Conversion(#[from] types::error::ConversionError),

    #[error("Cloudflare API returned no result for {0}")]
    MissingResult(&'static str),

    #[error("Cloudflare API error: {0}")]
    Api(String),
}

impl<T> From<zone::Error<T>> for CloudflareError
where
    T: std::fmt::Debug,
{
    fn from(value: zone::Error<T>) -> Self {
        Self::Api(format!("{value:?}"))
    }
}
