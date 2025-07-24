use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyToolCallResponseJsonModel {
    pub appointment_date: String,
    pub appointment_time: String,
}
