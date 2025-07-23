use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyToolCallResponseJsonModel {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub amenities: Option<Vec<String>>,
    pub usage_type: String,
    pub unit_type: String,
    pub unit_price: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_in_usd: Option<f64>,

    pub expected_completion_year: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apartment_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sq_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sq_feet: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub balcony_area_sq_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balcony_area_sq_feet: Option<f64>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // pub unit_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saleable_area_sq_meeters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saleable_area_sq_feet: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_latitude: Option<f64>,

    pub project_currency: String,
    pub country: String,
    pub city: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_slots: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_bedrooms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_bathrooms: Option<u32>,
}
