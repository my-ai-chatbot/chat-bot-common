//todo("Delete me")

use rust_extensions::sorted_vec::EntityWithStrKey;
use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectJsonModel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub rich_description: Option<String>,
    pub image: String,
    #[serde(rename = "type")]
    pub project_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    pub project_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amenities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why_invest: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_floors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sqm: Option<String>,
}

impl EntityWithStrKey for ProjectJsonModel {
    fn get_key(&self) -> &str {
        &self.id
    }
}
