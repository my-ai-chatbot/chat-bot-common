use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectJsonModel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub project_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amenities: Option<Vec<String>>,
}
