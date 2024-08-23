use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareZoneRecordsResponse {
    pub result: Vec<Record>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    #[serde(rename = "result_info")]
    pub result_info: ResultInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub proxied: bool,
    pub ttl: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "auto_added")]
    pub auto_added: bool,
    #[serde(rename = "managed_by_apps")]
    pub managed_by_apps: bool,
    #[serde(rename = "managed_by_argo_tunnel")]
    pub managed_by_argo_tunnel: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultInfo {
    pub page: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub count: i64,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}
