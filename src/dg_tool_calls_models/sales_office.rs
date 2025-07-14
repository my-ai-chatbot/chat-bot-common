use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOfficeJsonModel {
    pub city: String,
    pub addresses: Vec<String>,
    pub phones: Vec<String>,
    pub img_url: String,
}
