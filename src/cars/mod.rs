use serde::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct CarInventoryModel {
    pub id: String,
    pub car_brand: String,
    pub model: String,
    pub price: f64,
    pub year: i64,
    pub mileage: f64,
    pub drive_train: Option<String>,
    pub efficiency_label: Option<String>,
    pub engine_type: Option<String>,
    pub fuel: Option<String>,
    pub transmission: Option<String>,
    pub options: Vec<String>,
}
