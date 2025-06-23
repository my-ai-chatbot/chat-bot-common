pub const SST_ELEVEN_LABS: &'static str = "11labs";
pub const SST_KYUTAI: &'static str = "kyutai";
pub const SST_MUNSIT: &'static str = "munsit";

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
