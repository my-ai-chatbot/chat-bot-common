use std::str::FromStr;

use serde::*;

pub const GPT_4O: &'static str = "gpt-4o";
pub const GPT_4O_MINI: &'static str = "gpt-4o-mini";
pub const GPT_5: &'static str = "gpt-5";
pub const GPT_5_MINI: &'static str = "gpt-5-mini";
pub const GPT_5_NANO: &'static str = "gpt-5-nano";
pub const QWEN3_30B_A3B: &'static str = "Qwen3-30B-A3B";
pub const ZAI_GLM_4_5: &'static str = "zai-org-glm-4.5";
pub const ZAI_GLM_4_5_AIR: &'static str = "zai-glm-4.5-air";
pub const ZAI_GLM_4_5_X: &'static str = "zai-glm-4.5-x";
pub const ZAI_GLM_4_6: &'static str = "zai-org-glm-4.6";
pub const OPEN_AI_AGENT: &'static str = "open-ai-agent";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChatBotLlmModel {
    Gpt4o,
    Gpt4oMini,
    Gpt5,
    Gpt5Mini,
    Gpt5Nano,
    Qwen3_30BA3n,
    ZaiGlm4_5,
    ZaiGlm4_5Air,
    ZaiGlm4_5X,
    ZaiGlm4_6,
    OpenAiAgent,
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
        Self::OpenAiAgent,
        Self::Qwen3_30BA3n,
        Self::ZaiGlm4_5,
        Self::ZaiGlm4_5Air,
        Self::ZaiGlm4_5X,
        Self::ZaiGlm4_6,
    ];

    pub fn is_open_ai_agent(&self) -> bool {
        match self {
            ChatBotLlmModel::OpenAiAgent => true,
            _ => false,
        }
    }

    pub fn as_generic_llm_type(&self) -> ChatBotLlmGenericType {
        match self {
            ChatBotLlmModel::Gpt4o => ChatBotLlmGenericType::Gpt4,
            ChatBotLlmModel::Gpt4oMini => ChatBotLlmGenericType::Gpt4,
            ChatBotLlmModel::Gpt5 => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Gpt5Mini => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Gpt5Nano => ChatBotLlmGenericType::Gpt5,
            ChatBotLlmModel::Qwen3_30BA3n => ChatBotLlmGenericType::Qwen,
            ChatBotLlmModel::OpenAiAgent => ChatBotLlmGenericType::OpenAiAgent,
            ChatBotLlmModel::ZaiGlm4_5 => ChatBotLlmGenericType::Zai,
            ChatBotLlmModel::ZaiGlm4_5Air => ChatBotLlmGenericType::Zai,
            ChatBotLlmModel::ZaiGlm4_5X => ChatBotLlmGenericType::Zai,
            ChatBotLlmModel::ZaiGlm4_6 => ChatBotLlmGenericType::Zai,
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
            OPEN_AI_AGENT => Some(Self::OpenAiAgent),
            ZAI_GLM_4_5 => Some(Self::ZaiGlm4_5),
            ZAI_GLM_4_5_AIR => Some(Self::ZaiGlm4_5Air),
            ZAI_GLM_4_5_X => Some(Self::ZaiGlm4_5X),
            ZAI_GLM_4_6 => Some(Self::ZaiGlm4_6),
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
            Self::OpenAiAgent => OPEN_AI_AGENT,
            Self::ZaiGlm4_5 => ZAI_GLM_4_5,
            Self::ZaiGlm4_6 => ZAI_GLM_4_6,
            Self::ZaiGlm4_5Air => ZAI_GLM_4_5_AIR,
            Self::ZaiGlm4_5X => ZAI_GLM_4_5_X,
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
    OpenAiAgent,
    Zai,
}

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for ChatBotLlmModel {
    type TItem = Self;

    fn get_value(&self) -> Self
    where
        Self: Sized,
    {
        *self
    }

    fn get_all() -> &'static [Self::TItem]
    where
        Self: Sized,
    {
        Self::ALL
    }
}

impl rust_extensions::AsStr for ChatBotLlmModel {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for ChatBotLlmModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::try_from_str(s) {
            Some(value) => return Ok(value),
            None => return Err(format!("Invalid '{}' value to parse ChatBotLlmModel", s)),
        }
    }
}
