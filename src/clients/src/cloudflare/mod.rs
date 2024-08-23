use reqwest::{StatusCode};
use crate::cloudflare::error::Error;
use crate::cloudflare::types::{CloudflareZoneRecordsResponse, Record};
use crate::exponential_backoff;

pub mod types;
pub mod error;

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) zone_id: String,
    pub(crate) api_url: String,
    pub(crate) proxy_enabled: bool,
}

impl Client {
    pub fn new(api_url: String, api_key: String, zone_id: String, proxy_enabled: bool) -> Self {
        Self {
            api_key,
            zone_id,
            api_url,
            proxy_enabled,
        }
    }

    pub async fn get_zone_records(&self) -> Result<Vec<Record>, Error> {
        let api_url = format!("{}/zones/{}/dns_records", self.api_url, self.zone_id);

        let request_builder = reqwest::Client::new()
            .get(api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key));

        let domain_records = exponential_backoff::request(request_builder).await.map_err(|error| {
            tracing::error!("Failed to fetch domain records: {}", error);
            Error::new("Failed to fetch domain records".to_string())
        })?;

        let status: StatusCode = domain_records.status();
        let body: String = domain_records.text().await.unwrap_or_else(|_| "Empty response body".to_string());

        tracing::debug!("Received response {}, {}", status, body);

        if status.is_success() {
            let response: CloudflareZoneRecordsResponse = serde_json::from_str(&body).map_err(|error| {
                tracing::error!("Failed to parse response body: {}", error);
                Error::new("Failed to parse response body".to_string())
            })?;

            let all_records = response.result;

            let a_records: Vec<Record> = all_records.into_iter().filter(|record| record.type_field == "A").collect();

            Ok(a_records)
        } else {
            Err(Error::new("Failed to fetch domain records".to_string()))
        }
    }

    pub async fn update_zone_record(&self, id: &str, name: &str, content: &str) -> Result<(), Error> {
        let url = format!(
            "{}/zones/{}/dns_records/{}",
            self.api_url,
            self.zone_id,
            id
        );

        let body = serde_json::json!({
            "type": "A",
            "name": name,
            "content": content,
            "proxied": self.proxy_enabled,
        });

        let request_builder = reqwest::Client::new()
            .patch(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body);

        let response = exponential_backoff::request(request_builder).await.map_err(|error| {
            tracing::error!("Failed to update record: {}", error);
            Error::new("Failed to update record".to_string())
        })?;

        let status = response.status();
        let message = response.text().await.unwrap_or_else(|_| "Empty response body".to_string());

        tracing::debug!("Received response {}, {}", status, message);

        if status.is_success() {
            Ok(())
        } else {
            Err(Error::new("Failed to update record".to_string()))
        }

    }

    pub async fn create_zone_record(&self, name: &str, content: &str) -> Result<(), Error> {
        let url = format!(
            "{}/zones/{}/dns_records",
            self.api_url,
            self.zone_id,
        );

        let body = serde_json::json!({
            "type": "A",
            "name": name,
            "content": content,
            "proxied": self.proxy_enabled,
        });

        let request_builder = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body);

        let response = exponential_backoff::request(request_builder).await.map_err(|error| {
            tracing::error!("Failed to create record: {}", error);
            Error::new("Failed to create record".to_string())
        })?;

        let status = response.status();

        let message = response.text().await.unwrap_or_else(|_| "Empty response body".to_string());

        tracing::debug!("Received response {}, {}", status, message);

        if status.is_success() {
            Ok(())
        } else {
            Err(Error::new("Failed to create record".to_string()))
        }
    }
}
