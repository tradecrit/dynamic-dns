use std::env;
use std::str::FromStr;

// Top level provider configuration
#[derive(Debug, Clone)]
pub struct DnsProvider {
    pub config: DnsProviderSelection,
}

// The supported DNS providers, currently only Cloudflare
#[derive(Debug, Clone)]
pub enum DnsProviderSelection {
    Cloudflare(CloudflareProviderSettings),
}

// Implement the FromStr trait for DnsProviderConfig, essentially based on the selected provider
// we will load the required environment variables, and the required environment variables are
// different for each provider
impl FromStr for DnsProviderSelection {
    type Err = String;

    fn from_str(raw_input: &str) -> Result<Self, Self::Err> {
        let binding = raw_input.to_lowercase();
        let input = binding.as_str();

        match input {
            "cloudflare" => {
                tracing::debug!("Loading Cloudflare provider settings");
                let api_url: String = env::var("CLOUDFLARE_API_URL")
                    .unwrap_or("https://api.cloudflare.com/client/v4".to_string());

                let zone_id: String = env::var("CLOUDFLARE_ZONE_ID").expect("CLOUDFLARE_ZONE_ID must be set");
                let api_key: String = env::var("CLOUDFLARE_API_KEY").expect("CLOUDFLARE_API_KEY must be set");

                let proxy_enabled: bool = env::var("CLOUDFLARE_PROXY_ENABLED")
                    .unwrap_or("false".to_string())
                    .parse()
                    .expect("CLOUDFLARE_PROXY_ENABLED must be a boolean");

                let settings = CloudflareProviderSettings::new(
                    zone_id,
                    api_url,
                    api_key,
                    proxy_enabled
                );

                Ok(DnsProviderSelection::Cloudflare(settings))
            },
            _ => Err("Invalid DNS provider".to_string())
        }
    }
}


#[derive(Debug, Clone)]
pub struct CloudflareProviderSettings {
    pub zone_id: String,
    pub api_url: String,
    pub api_key: String,
    pub proxy_enabled: bool,
}

impl CloudflareProviderSettings {
    pub fn new(zone_id: String, api_url: String, api_key: String, proxy_enabled: bool) -> Self {
        Self {
            zone_id,
            api_url,
            api_key,
            proxy_enabled,
        }
    }
}