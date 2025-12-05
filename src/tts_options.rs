use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::languages::Language;

pub const TTS_ELEVEN_LABS: &'static str = "11labs";
pub const TTS_KOKORO: &'static str = "kokoro";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TtsOption {
    ElevenLabs,
    Kokoro,
}

impl TtsOption {
    pub const ALL: &[Self] = &[Self::ElevenLabs, Self::Kokoro];
    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            TTS_ELEVEN_LABS => Self::ElevenLabs.into(),
            TTS_KOKORO => Self::Kokoro.into(),

            _ => None,
        }
    }

    pub fn from_str(src: &str, lang: Language) -> Self {
        match Self::try_from_str(src) {
            Some(value) => value,
            None => Self::get_default_by_language(&lang),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ElevenLabs => TTS_ELEVEN_LABS,
            Self::Kokoro => TTS_KOKORO,
        }
    }

    pub fn get_default_by_language(_lang_id: &Language) -> Self {
        Self::default()
    }
}

impl Default for TtsOption {
    fn default() -> Self {
        Self::Kokoro
    }
}

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for TtsOption {
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

impl rust_extensions::AsStr for TtsOption {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for TtsOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::try_from_str(s) {
            Some(value) => return Ok(value),
            None => return Err(format!("Invalid '{}' value to parse SttOption", s)),
        }
    }
}
