//! Module that contains the main [Ipbase] struct

use std::sync::Arc;
use reqwest::Client;
use crate::error::IpbaseError;
use crate::{error, models, utils};
use crate::utils::baseline::construct_base_url;

/// Settings struct that contains the api key
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
}

/// The main struct of the crate giving access to the ipbase.
/// Create a new instance of the struct with your api key as parameter.
#[derive(Debug, Clone)]
pub struct Ipbase {
    client: Client,
    settings: Arc<Settings>,
}

impl<'a> Ipbase {
    /// Creates a new instance of the Ipbase struct by passing your api key as
    /// function parameter.
    pub fn new(api_key: &'a str) -> Result<Self, IpbaseError> {
        let settings = std::sync::Arc::new(Settings {
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self { client, settings })
    }

    pub async fn status(
        &self,
    ) -> Result<models::DetailsResponse, error::IpbaseError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("status"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::IpbaseError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::IpbaseError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::IpbaseError::ResponseParsingError { body: res_body })
    }

    pub async fn info(
        &self,
        ip: &'a str,
        language: &'a str,
    ) -> Result<models::DetailsResponse, error::IpbaseError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("latest"))?;
        url.query_pairs_mut()
            .append_pair("ip", ip)
            .append_pair("language", language);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::IpbaseError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::IpbaseError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::IpbaseError::ResponseParsingError { body: res_body })
    }
}
