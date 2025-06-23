pub const SST_ELEVEN_LABS: &'static str = "11labs";
pub const SST_KYUTAI: &'static str = "kyutai";

pub enum SttOption {
    ElevenLabs,
    Kyutai,
}

impl Default for SttOption {
    fn default() -> Self {
        Self::ElevenLabs
    }
}

pub const ALL: &[&'static str] = &[SST_ELEVEN_LABS, SST_KYUTAI];
