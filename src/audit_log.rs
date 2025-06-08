use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const AUDIT_LOG_CHAT_EVENT: &'static str = "chat-event";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct AuditLogChatModel {
    pub client_question: String,
    pub llm_answer: String,
    pub ai_agent_duration: Option<String>,

    pub stt_audio_file: Option<String>,
    pub stt_duration: Option<String>,
    pub stt: Option<String>,

    pub req_time: Option<String>,

    pub tts_audio_file: Option<String>,
    pub tts_duration: Option<String>,
    pub tts: Option<String>,

    pub resp_time: Option<String>,
}

impl AuditLogChatModel {
    pub fn get_req_time(&self) -> Option<DateTimeAsMicroseconds> {
        let req_time = self.req_time.as_ref()?;
        DateTimeAsMicroseconds::from_str(req_time)
    }

    pub fn get_resp_time(&self) -> Option<DateTimeAsMicroseconds> {
        let resp_time = self.resp_time.as_ref()?;
        DateTimeAsMicroseconds::from_str(resp_time)
    }
}
