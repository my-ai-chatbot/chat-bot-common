use serde::*;

pub trait LlmAgentGenericSettings {
    fn get_temperature(&self) -> Option<f64>;
    fn get_top_p(&self) -> Option<f64>;
    fn get_n(&self) -> Option<i64>;
    fn get_presence_penalty(&self) -> Option<f64>;
    fn get_frequency_penalty(&self) -> Option<f64>;
    fn get_disable_think(&self) -> Option<bool>;
    fn get_reasoning_effort(&self) -> Option<Gpt5ReasoningEffort>;
    fn get_verbosity(&self) -> Option<Gpt5VerbosityEffort>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Gpt5ReasoningEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Gpt5VerbosityEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}
