pub enum ClientDataType {
    MinaghiAuto,
    DarGlobalRealEstate,
    SalesTeq,
}

impl ClientDataType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            MINAGHI_AUTO => Self::MinaghiAuto.into(),
            DG_REAL_ESTATE => Self::DarGlobalRealEstate.into(),
            SALESTEQ => Self::SalesTeq.into(),
            _ => None,
        }
    }
}

pub const MINAGHI_AUTO: &'static str = "minaghi-auto";
pub const DG_REAL_ESTATE: &'static str = "dg-real-estate";
pub const SALESTEQ: &'static str = "salesteq";
pub const SUHER: &'static str = "suher";

pub const ALL: &[&'static str] = &[MINAGHI_AUTO, DG_REAL_ESTATE, SALESTEQ, SUHER];
