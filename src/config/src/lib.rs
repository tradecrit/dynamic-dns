pub mod dns_providers;

use std::env;
use dotenvy::dotenv;

use crate::dns_providers::{DnsProvider, DnsProviderSelection};

#[derive(Debug, Clone)]
pub struct AppState {
    pub environment: String,
    pub dns_entries_to_sync: Vec<String>,
    pub domain: String,
    pub refresh_interval_seconds: u64,
    pub dns_provider: DnsProvider
}

trait StripQuotes {
    fn strip_quotes(&self) -> String;
}

impl StripQuotes for String {
    fn strip_quotes(&self) -> String {
        self.trim_matches(|c| c == '\'' || c == '"').to_string()
    }
}

fn init_observability(log_level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(log_level)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true)
        )
        .json()
        .init();
}

pub async fn load_state() -> AppState {
    // Log configuration and bootstrap
    let load_env = dotenv();
    if load_env.is_err() {
        tracing::warn!("No .env file found");
    }

    let raw_log_level: String = env::var("RUST_LOG").unwrap_or_else(|_| "INFO".to_string()).strip_quotes();
    let uppercased_log_level: String = raw_log_level.to_uppercase();

    let tracing_level: tracing::Level = match uppercased_log_level.as_str() {
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    init_observability(tracing_level);

    tracing::info!("Starting application with tracing level: {}", tracing_level);

    // Core environment variables
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()).strip_quotes();

    let domain: String = env::var("DOMAIN").expect("DOMAIN must be set").strip_quotes();

    let raw_dns_entries_to_sync: String = env::var("DNS_ENTRIES_TO_SYNC").unwrap_or_default();
    let dns_entries_to_sync: Vec<String> = raw_dns_entries_to_sync
        .strip_quotes()
        .split(',')
        .map(|s| s.to_string()).collect();

    let refresh_interval_seconds: u64 = env::var("REFRESH_INTERVAL_SECONDS")
        .unwrap_or_else(|_| String::from("60"))
        .strip_quotes()
        .parse()
        .expect("REFRESH_INTERVAL_SECONDS must be a number");

    let dns_provider_selection: DnsProviderSelection = env::var("DNS_PROVIDER")
        .expect("DNS_PROVIDER must be set")
        .strip_quotes()
        .parse()
        .expect("Invalid DNS_PROVIDER value");

    let dns_provider: DnsProvider = DnsProvider {
        config: dns_provider_selection
    };


    // for each strip all single and double quote from start/end if present
    let app_state: AppState = AppState {
        environment,
        domain,
        dns_entries_to_sync,
        dns_provider,
        refresh_interval_seconds
    };

    app_state
}
