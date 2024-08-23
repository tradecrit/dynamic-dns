use std::fmt::Display;
use serde::Deserialize;
use crate::exponential_backoff;

#[derive(Debug, Clone)]
pub enum Format {
    Json,
    Text,
}

#[derive(Debug, Clone)]
pub struct Ipify {
    url: &'static str,
    format: Format,
}

#[derive(Debug, Clone, Deserialize)]
struct IpifyResponse {
    ip: String,
}

impl Ipify {
    pub fn new(format: Format) -> Self {
        let url = match format {
            Format::Json => {
                "https://api.ipify.org?format=json"
            }
            Format::Text => {
                "https://api.ipify.org"
            }
        };

        Self {
            format,
            url,
        }
    }

    pub async fn get_ip(&self) -> Result<String, reqwest::Error> {
        tracing::debug!("Fetching public IP address from: {}", self.url);

        let fetch_ip_request = reqwest::Client::new()
            .get(self.url);

        let fetch_ip = exponential_backoff::request(fetch_ip_request).await;

        let ip_address = fetch_ip.expect("Failed to fetch IP address");

        match self.format {
            Format::Json => {
                let ipify_response: IpifyResponse = ip_address.json().await.expect("Failed to parse JSON response");

                tracing::debug!("Fetched public IP address: {}", ipify_response.ip);

                Ok(ipify_response.ip)
            }
            Format::Text => {
                let ip_address = ip_address.text().await.expect("Failed to parse text response");

                tracing::debug!("Fetched public IP address: {}", ip_address);

                Ok(ip_address)
            }
        }
    }
}

impl Display for Ipify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url: {}", self.url)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ip_text() {
        let ipify: Ipify = Ipify::new(Format::Text);

        let result = ipify.get_ip().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_ip_json() {
        let ipify: Ipify = Ipify::new(Format::Json);

        let result = ipify.get_ip().await;

        assert!(result.is_ok());
    }
}