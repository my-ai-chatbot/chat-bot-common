use std::str::FromStr;

use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Gpt5ReasoningEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    #[default]
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

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for Gpt5ReasoningEffort {
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

impl rust_extensions::AsStr for Gpt5ReasoningEffort {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for Gpt5ReasoningEffort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::try_from_str(s) {
            Some(value) => return Ok(value),
            None => {
                return Err(format!(
                    "Invalid '{}' value to parse Gpt5ReasoningEffort",
                    s
                ));
            }
        }
    }
}
