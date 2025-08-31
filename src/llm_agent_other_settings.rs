pub trait LlmAgentOtherSettings {
    fn get_temperature(&self) -> Option<f64>;
    fn get_top_p(&self) -> Option<f64>;
    fn get_top_k(&self) -> Option<i64>;
    fn get_n(&self) -> Option<i64>;
    fn get_presence_penalty(&self) -> Option<f64>;
    fn get_frequency_penalty(&self) -> Option<f64>;
    fn get_disable_think(&self) -> Option<bool>;
}
