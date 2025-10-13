pub trait LlmAgentGenericSettings {
    fn get_temperature(&self) -> Option<f64>;
    fn get_top_p(&self) -> Option<f64>;
    fn get_n(&self) -> Option<i64>;
    fn get_presence_penalty(&self) -> Option<f64>;
    fn get_frequency_penalty(&self) -> Option<f64>;
    fn get_think(&self) -> bool;
    fn get_reasoning_effort(&self) -> super::Gpt5ReasoningEffort;
    fn get_verbosity(&self) -> super::Gpt5Verbosity;
}
