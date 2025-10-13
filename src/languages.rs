use serde::*;

pub const LANG_EN: &'static str = "en";
pub const LANG_AR: &'static str = "ar";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Language {
    En,
    Ar,
}

impl Language {
    pub const ALL_LANGUAGES: &'static [Self] = &[Self::En, Self::Ar];

    pub fn is_english(&self) -> bool {
        match self {
            Self::En => true,
            _ => false,
        }
    }

    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            LANG_EN => Some(Self::En),
            LANG_AR => Some(Self::Ar),
            _ => None,
        }
    }

    pub fn from_str(src: &str) -> Self {
        if let Some(lang) = Self::try_from_str(src) {
            return lang;
        }

        Self::default()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => LANG_EN,
            Self::Ar => LANG_AR,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for Language {
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
        Self::ALL_LANGUAGES
    }
}

#[cfg(feature = "dioxus")]
impl rust_extensions::AsStr for Language {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}
