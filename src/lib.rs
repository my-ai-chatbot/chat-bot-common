pub mod ai_model_type;
pub mod audit_log;
pub mod browsers;
pub mod currency_converter;
pub mod dg_tool_calls_models;
pub mod inventory_type;
pub mod languages;
pub mod request_metadata;
pub mod stt_options;
pub mod translations;
pub use currency_converter::*;
pub mod tool_call_resp_models;

pub trait EnumAsStr {
    fn try_from_str(src: &str) -> Option<Self>
    where
        Self: Sized;

    fn as_str(&self) -> &'static str;

    fn get_value(&self) -> Self
    where
        Self: Sized;

    fn get_all(&self) -> impl Iterator<Item = Self>
    where
        Self: Sized;
}
