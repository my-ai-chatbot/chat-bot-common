use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyToolCallResponseJsonModel {
    pub appointment_time_zone: String,
}
