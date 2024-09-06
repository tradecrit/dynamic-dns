use std::collections::HashMap;
use clients::cloudflare;
use clients::ipify::{Format, Ipify};
use crate::error::Error;
use config::dns_providers::{DnsProvider};
use config::dns_providers::DnsProviderSelection::Cloudflare;
use crate::dns_record::DnsRecord;

pub mod error;
pub mod dns_record;

/// Resolves the public IP address of the current machine, using the Ipify service.
///
/// # Returns
/// String containing the public IP address.
pub async fn resolve_public_ip() -> Result<String, Error> {
    let fetch_ip_address = Ipify::new(Format::Text).get_ip().await;

    let ip_address: String = fetch_ip_address.map_err(|error| {
        tracing::error!("Failed to fetch public IP address: {:?}", error);
        Error::new("Failed to fetch public IP address".to_string())
    })?;

    tracing::info!("Resolved Public IP: {}", ip_address);

    Ok(ip_address)
}

/// Fetches the existing DNS records from the DNS provider.
///
/// # Arguments
/// * `dns_provider` - The DNS provider configuration.
///
/// # Returns
/// A HashMap containing the existing DNS records.
pub async fn get_dns_records(
    dns_provider: &DnsProvider
) -> Result<HashMap<String, DnsRecord>, Error> {
    match &dns_provider.config {
        Cloudflare(provider) => {
            tracing::debug!("Fetching DNS records from Cloudflare");

            let api_url = &provider.api_url;
            let api_key = &provider.api_key;
            let zone_id = &provider.zone_id;

            let client = cloudflare::Client::new(
                api_url.clone(),
                api_key.clone(),
                zone_id.clone(),
                provider.proxy_enabled
            );

            let fetch_domain_records = client.get_zone_records().await;

            let dns_records = match fetch_domain_records {
                Ok(records) => records,
                Err(error) => {
                    tracing::error!("Failed to fetch domain records: {:?}", error);
                    return Err(Error::new("Failed to fetch domain records".to_string()));
                }
            };

            let mut dns_records_map: HashMap<String, DnsRecord> = HashMap::new();
            dns_records.iter().for_each(|record| {
                dns_records_map.insert(record.name.clone(), record.clone().into());
            });

            tracing::info!("Found {} existing A records", dns_records.len());
            tracing::debug!("Existing A records: {:?}", dns_records);

            Ok(dns_records_map)
        }
    }
}

pub async fn ensure_dns_record(
    dns_provider: &DnsProvider,
    record: &DnsRecord
) -> Result<(), Error> {
    match &dns_provider.config {
        Cloudflare(provider) => {
            let api_url = provider.api_url.clone();
            let api_key = provider.api_key.clone();
            let zone_id = provider.zone_id.clone();
            let proxy_enabled = provider.proxy_enabled.clone();

            let client = cloudflare::Client::new(api_url, api_key, zone_id, proxy_enabled);

            let updated_record = match &record.id {
                Some(id) => {
                    client.update_zone_record(&id, &record.name, &record.content).await
                },
                None => {
                    client.create_zone_record(&record.name, &record.content).await
                }
            };

            match updated_record {
                Ok(_) => {
                    tracing::info!("Record updated successfully");
                    Ok(())
                },
                Err(error) => {
                    tracing::error!("Failed to update record: {:?}", error);
                    Err(Error::new("Failed to update record".to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_public_ip_text() {
        let ipify: Ipify = Ipify::new(Format::Text);

        let result = ipify.get_ip().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resolve_public_ip_json() {
        let ipify: Ipify = Ipify::new(Format::Json);

        let result = ipify.get_ip().await;

        assert!(result.is_ok());
    }
}
