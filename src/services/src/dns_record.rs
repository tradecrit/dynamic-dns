use std::fmt::Display;
use clients::cloudflare::types::Record;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsRecord {
    pub id: Option<String>,
    pub name: String,
    pub content: String,
}

impl From<Record> for DnsRecord {
    fn from(record: Record) -> Self {
        Self {
            id: record.id,
            name: record.name,
            content: record.content,
        }
    }
}

impl Display for DnsRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ID: {}, Name: {}, Content: {}", self.id.clone().unwrap_or_else(|| "None".to_string()), self.name, self.content)
    }
}

impl DnsRecord {
    /// Build a new DNS record or update an existing one.
    ///
    /// # Arguments
    /// subdomain - The subdomain to create or update.
    /// ip - The IP address to set for the record.
    /// existing_record - The existing record to update, if it exists.
    ///
    /// # Returns
    /// A new DNS record.
    pub fn build_record(subdomain: &str, ip: &str, existing_record: Option<&DnsRecord>) -> DnsRecord {
        match existing_record {
            Some(record) => {
                DnsRecord {
                    id: record.id.clone(),
                    name: record.name.clone(),
                    content: ip.to_string(),
                }
            },
            None => {
                DnsRecord {
                    id: None,
                    name: subdomain.to_string(),
                    content: ip.to_string(),
                }
            },
        }
    }
}
