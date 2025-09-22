use serde::*;

pub trait LlmAgentGenericSettings {
    fn get_temperature(&self) -> Option<f64>;
    fn get_top_p(&self) -> Option<f64>;
    fn get_n(&self) -> Option<i64>;
    fn get_presence_penalty(&self) -> Option<f64>;
    fn get_frequency_penalty(&self) -> Option<f64>;
    fn get_think(&self) -> bool;
    fn get_reasoning_effort(&self) -> Option<Gpt5ReasoningEffort>;
    fn get_verbosity(&self) -> Option<Gpt5Verbosity>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Gpt5ReasoningEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
}

impl Gpt5ReasoningEffort {
    pub const ALL: &'static [Self] = &[Self::Hight, Self::Low, Self::Minimal];

    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            "high" => Some(Self::Hight),
            "minimal" => Some(Self::Minimal),
            "low" => Some(Self::Low),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hight => "high",
            Self::Minimal => "minimal",
            Self::Low => "low",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Gpt5Verbosity {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

impl Gpt5Verbosity {
    pub const ALL: &'static [Self] = &[Self::Hight, Self::Medium, Self::Low];
    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            "high" => Some(Self::Hight),
            "medium" => Some(Self::Medium),
            "low" => Some(Self::Low),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hight => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
