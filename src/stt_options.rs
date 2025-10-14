use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::languages::Language;

pub const STT_ELEVEN_LABS: &'static str = "11labs";
pub const STT_KYUTAI: &'static str = "kyutai";
pub const STT_MUNSIT: &'static str = "munsit";
pub const STT_OPEN_AI: &'static str = "open_ai";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SttOption {
    OpenAi,
    ElevenLabs,
    Kyutai,
    Munsit,
}

impl SttOption {
    pub const ALL: &[Self] = &[
        SttOption::OpenAi,
        SttOption::ElevenLabs,
        SttOption::Kyutai,
        SttOption::Munsit,
    ];
    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            STT_OPEN_AI => Self::OpenAi.into(),
            STT_ELEVEN_LABS => Self::ElevenLabs.into(),
            STT_KYUTAI => Self::Kyutai.into(),
            STT_MUNSIT => Self::Munsit.into(),
            _ => None,
        }
    }

    pub fn from_str(src: &str, lang: Language) -> Self {
        match Self::try_from_str(src) {
            Some(value) => value,
            None => get_default_by_language(&lang),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SttOption::OpenAi => STT_OPEN_AI,
            SttOption::ElevenLabs => STT_ELEVEN_LABS,
            SttOption::Kyutai => STT_KYUTAI,
            SttOption::Munsit => STT_MUNSIT,
        }
    }
}

impl Default for SttOption {
    fn default() -> Self {
        Self::ElevenLabs
    }
}

pub fn get_default_by_language(lang_id: &Language) -> SttOption {
    match lang_id {
        Language::En => SttOption::OpenAi,
        Language::Ar => SttOption::Munsit,
    }
}

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for SttOption {
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

impl rust_extensions::AsStr for SttOption {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for SttOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::try_from_str(s) {
            Some(value) => return Ok(value),
            None => return Err(format!("Invalid '{}' value to parse SttOption", s)),
        }
    }
}
