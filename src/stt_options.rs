use crate::languages::Language;

pub const SST_ELEVEN_LABS: &'static str = "11labs";
pub const SST_KYUTAI: &'static str = "kyutai";
pub const SST_MUNSIT: &'static str = "munsit";

#[derive(Debug, Clone, Copy)]
pub enum SttOption {
    ElevenLabs,
    Kyutai,
    Munsit,
}

impl SttOption {
    pub fn from_str(src: &str) -> Self {
        match src {
            SST_ELEVEN_LABS => Self::ElevenLabs,
            SST_KYUTAI => Self::Kyutai,
            SST_MUNSIT => Self::Munsit,
            _ => Default::default(),
        }
    }
}

impl Default for SttOption {
    fn default() -> Self {
        Self::ElevenLabs
    }
}

pub const ALL: &[&'static str] = &[SST_ELEVEN_LABS, SST_KYUTAI, SST_MUNSIT];

pub fn get_default(lang_id: &Language) -> SttOption {
    match lang_id {
        Language::En => SttOption::ElevenLabs,
        Language::Ar => SttOption::Munsit,
    }
}
