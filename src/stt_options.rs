use serde::{Deserialize, Serialize};

use crate::languages::Language;

pub const STT_ELEVEN_LABS: &'static str = "11labs";
pub const STT_KYUTAI: &'static str = "kyutai";
pub const STT_MUNSIT: &'static str = "munsit";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SttOption {
    ElevenLabs,
    Kyutai,
    Munsit,
}

impl SttOption {
    pub const ALL: &[Self] = &[SttOption::ElevenLabs, SttOption::Kyutai, SttOption::Munsit];
    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            STT_ELEVEN_LABS => Self::ElevenLabs.into(),
            STT_KYUTAI => Self::Kyutai.into(),
            STT_MUNSIT => Self::Munsit.into(),
            _ => None,
        }
    }

    pub fn from_str(src: &str, lang: Language) -> Self {
        match src {
            STT_ELEVEN_LABS => Self::ElevenLabs,
            STT_KYUTAI => Self::Kyutai,
            STT_MUNSIT => Self::Munsit,
            _ => get_default_by_language(&lang),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
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
        Language::En => SttOption::ElevenLabs,
        Language::Ar => SttOption::Munsit,
    }
}
