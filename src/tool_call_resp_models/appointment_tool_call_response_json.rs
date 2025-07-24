use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppointmentToolCallResponseJsonModel {
    pub appointment_date: String,
    pub appointment_time: String,
}
