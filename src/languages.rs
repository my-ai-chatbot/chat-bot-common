pub const LANG_EN: &'static str = "en";
pub const LANG_AR: &'static str = "ar";

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
    Ar,
}

impl Language {
    pub const ALL_LANGUAGES: &[Self] = &[Self::En, Self::Ar];

    pub fn from_str(src: &str) -> Self {
        match src {
            LANG_AR => Self::Ar,
            _ => Self::default(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => LANG_EN,
            Self::Ar => LANG_AR,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}
