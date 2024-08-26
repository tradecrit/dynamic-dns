mod app_error;

use config::AppState;
use services::dns_record::DnsRecord;
use services::error::Error;
use crate::app_error::AppError;

async fn workflow(config: &AppState) -> Result<(), AppError> {
    let fetch_public_ip: Result<String, Error> = services::resolve_public_ip().await;
    let public_ip: String = match fetch_public_ip {
        Ok(ip) => ip,
        Err(error) => {
            tracing::error!("Failed to fetch public IP address: {:?}", error);
            return Err(AppError::new("Failed to fetch public IP address".to_string()));
        }
    };

    let fetch_a_records = services::get_dns_records(&config.dns_provider).await;

    let a_records = match fetch_a_records {
        Ok(records) => records,
        Err(error) => {
            tracing::error!("Failed to fetch domain records: {:?}", error);
            return Err(AppError::new("Failed to fetch domain records".to_string()));
        }
    };

    tracing::debug!("A Records: {:?}", a_records.keys());

    // Check each subdomain to see if it exists in the map. If it exists, ensure the IP is correct.
    // If it is correct, do nothing. If it is incorrect, update the record.
    // If it is missing then create the record, We do not remove or touch any other records.
    let sync_entries = config.dns_entries_to_sync.clone();

    let mut update_records: Vec<DnsRecord> = vec![];

    for entry in &sync_entries {
        tracing::debug!("Checking entry: {}", entry);
        let fqdn = format!("{}.{}", entry, config.domain);

        if !a_records.contains_key(&fqdn) {
            tracing::info!("Missing record: {}", entry);
            let new_record = DnsRecord::build_record(entry, &public_ip, None);
            update_records.push(new_record);
        }

        if let Some(record) = a_records.get(&fqdn) {
            let updated_record = DnsRecord::build_record(entry, &public_ip, Some(record));

            tracing::debug!("Checking existing record: {}", record);
            tracing::debug!("Checking against desired record: {}", updated_record);

            if updated_record != *record {
                update_records.push(updated_record);
            }
        }
    }

    tracing::info!("{} records need to update", update_records.len());
    tracing::debug!("Records to update: {:?}", update_records);

    for record in &update_records {
        tracing::info!("Updating record: {}", record.name);

        let update_results = services::ensure_dns_record(&config.dns_provider, record).await;

        match update_results {
            Ok(_) => {
                tracing::info!("{} record updated to IP {}", record.name, public_ip);
            },
            Err(error) => {
                tracing::error!("Failed to update record {}: {:?}", record.name, error);
            }
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = config::load_state().await;

    tracing::info!("Starting application");
    tracing::info!("Refresh check interval set to {} seconds", config.refresh_interval_seconds);

    loop {
        let execution_result = workflow(&config).await;

        match execution_result {
            Ok(_) => {
                tracing::info!("Iteration successful");
            },
            Err(error) => {
                tracing::error!("Iteration failed: {:?}", error);
            }
        }

        tracing::info!("{}", format!("Iteration complete, waiting {} seconds until next run", config.refresh_interval_seconds));

        tokio::time::sleep(tokio::time::Duration::from_secs(config.refresh_interval_seconds)).await;
    }
}
