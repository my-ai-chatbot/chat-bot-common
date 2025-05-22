pub const LANG_EN: &'static str = "en";
pub const LANG_AR: &'static str = "ar";

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
    Ar,
}

impl Language {
    pub fn from_str(src: &str) -> Self {
        match src {
            LANG_EN => Self::Ar,
            _ => Self::En,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => LANG_EN,
            Self::Ar => LANG_AR,
        }
    }
}
