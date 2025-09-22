use serde::*;

pub const GPT_4O: &'static str = "gpt-4o";
pub const GPT_4O_MINI: &'static str = "gpt-4o-mini";
pub const GPT_5: &'static str = "gpt-5";
pub const GPT_5_MINI: &'static str = "gpt-5-mini";
pub const GPT_5_NANO: &'static str = "gpt-5-nano";
pub const QWEN3_30B_A3B: &'static str = "Qwen3-30B-A3B";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChatBotLlmModel {
    Gpt4o,
    Gpt4oMini,
    Gpt5,
    Gpt5Mini,
    Gpt5Nano,
    Qwen3_30BA3n,
}

impl Default for ChatBotLlmModel {
    fn default() -> Self {
        Self::Gpt4o
    }
}

impl ChatBotLlmModel {
    pub const ALL: &'static [Self] = &[
        Self::Gpt4o,
        Self::Gpt4oMini,
        Self::Gpt5,
        Self::Gpt5Mini,
        Self::Gpt5Nano,
        Self::Qwen3_30BA3n,
    ];

    pub fn as_generic_llm_type(&self) -> ChatBotLlmGenericType {
        match self {
            ChatBotLlmModel::Gpt4o => ChatBotLlmGenericType::Gpt4,
            ChatBotLlmModel::Gpt4oMini => ChatBotLlmGenericType::Gpt4,
            ChatBotLlmModel::Gpt5 => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Gpt5Mini => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Gpt5Nano => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Qwen3_30BA3n => ChatBotLlmGenericType::Qwen,
        }
    }

    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            GPT_4O => Some(Self::Gpt4o),
            GPT_4O_MINI => Some(Self::Gpt4oMini),
            GPT_5 => Some(Self::Gpt5),
            GPT_5_MINI => Some(Self::Gpt5Mini),
            GPT_5_NANO => Some(Self::Gpt5Nano),
            QWEN3_30B_A3B => Some(Self::Qwen3_30BA3n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gpt4o => GPT_4O,
            Self::Gpt4oMini => GPT_4O_MINI,
            Self::Gpt5 => GPT_5,
            Self::Gpt5Mini => GPT_5_MINI,
            Self::Gpt5Nano => GPT_5_NANO,
            Self::Qwen3_30BA3n => QWEN3_30B_A3B,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatBotLlmGenericType {
    Gpt4,
    Gpt5,
    Qwen,
}
