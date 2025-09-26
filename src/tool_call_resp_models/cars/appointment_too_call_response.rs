use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppointmentResult {
    MobileVerificationStarted,
    AppointmentScheduled,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppointmentToolCallResponse {
    pub result: AppointmentResult,
    pub message: String,
}
