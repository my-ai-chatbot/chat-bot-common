use serde::*;

use crate::*;

pub const GPT_4O: &'static str = "gpt-4o";
pub const GPT_4O_MINI: &'static str = "gpt-4o-mini";
pub const GPT_5: &'static str = "gpt-5";
pub const QWEN3_30B_A3B: &'static str = "Qwen3-30B-A3B";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChatBotLlmModel {
    Gpt4o,
    Gpt4oMini,
    Gpt5,
    Qwen3_30BA3n,
}

impl Default for ChatBotLlmModel {
    fn default() -> Self {
        Self::Gpt4o
    }
}

impl ChatBotLlmModel {
    pub const ALL: &'static [Self] =
        &[Self::Gpt4o, Self::Gpt4oMini, Self::Gpt5, Self::Qwen3_30BA3n];

    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            GPT_4O => Some(Self::Gpt4o),
            GPT_4O_MINI => Some(Self::Gpt4oMini),
            GPT_5 => Some(Self::Gpt5),
            QWEN3_30B_A3B => Some(Self::Qwen3_30BA3n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gpt4o => GPT_4O,
            Self::Gpt4oMini => GPT_4O_MINI,
            Self::Gpt5 => GPT_5,
            Self::Qwen3_30BA3n => QWEN3_30B_A3B,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl EnumIterator for ChatBotLlmModel {
    type TItem = Self;

    fn get_value(&self) -> Self
    where
        Self: Sized,
    {
        *self
    }

    fn get_all(&self) -> impl Iterator<Item = Self>
    where
        Self: Sized,
    {
        Self::ALL.iter().copied()
    }
}

impl ItemAsStr for ChatBotLlmModel {
    fn try_from_str(src: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::try_from_str(src)
    }

    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}
