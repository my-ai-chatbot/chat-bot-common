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

impl crate::EnumAsStr for Language {
    fn try_from_str(src: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::try_from_str(src)
    }

    fn get_value(&self) -> Self
    where
        Self: Sized,
    {
        *self
    }

    fn as_str(&self) -> &'static str {
        self.as_str()
    }

    fn get_all(&self) -> impl Iterator<Item = Self>
    where
        Self: Sized,
    {
        Self::ALL_LANGUAGES.iter().copied()
    }
}
