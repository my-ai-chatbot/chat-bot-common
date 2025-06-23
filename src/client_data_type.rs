pub enum ClientDataType {
    MinaghiAuto,
    DarGlobalRealEstate,
}

impl ClientDataType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            MINAGHI_AUTO => Self::MinaghiAuto.into(),
            DG_REAL_ESTATE => Self::DarGlobalRealEstate.into(),
            _ => None,
        }
    }
}

pub const MINAGHI_AUTO: &'static str = "minaghi-auto";
pub const DG_REAL_ESTATE: &'static str = "dg-real-estate";

pub const ALL: &[&'static str] = &[MINAGHI_AUTO, DG_REAL_ESTATE];
