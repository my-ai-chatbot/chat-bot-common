pub mod audit_log;
pub mod browsers;
pub mod currency_converter;
pub mod dg_tool_calls_models;
pub mod inventory_type;
pub mod languages;
pub mod llm_model_type;
pub mod request_metadata;
pub mod stt_options;
pub mod translations;
pub use currency_converter::*;
mod llm_agent_other_settings;
pub mod tool_call_resp_models;
pub use llm_agent_other_settings::*;

pub trait EnumIterator {
    type TItem;

    fn get_value(&self) -> Self::TItem;

    fn get_all(&self) -> impl Iterator<Item = Self::TItem>;
}

pub trait ItemAsStr {
    fn try_from_str(src: &str) -> Option<Self>
    where
        Self: Sized;
    fn as_str(&self) -> &'static str;
}
