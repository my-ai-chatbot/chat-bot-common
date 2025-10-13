use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Gpt5Verbosity {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    #[default]
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

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for Gpt5Verbosity {
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

#[cfg(feature = "dioxus")]
impl rust_extensions::AsStr for Gpt5Verbosity {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}
