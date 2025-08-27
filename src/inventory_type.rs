#[derive(Debug, Clone, Copy)]
pub enum InventoryType {
    MinaghiAuto,
    DarGlobalRealEstate,
    SalesTeq,
}

impl InventoryType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            MINAGHI_AUTO => Self::MinaghiAuto.into(),
            DG_REAL_ESTATE => Self::DarGlobalRealEstate.into(),
            SALESTEQ => Self::SalesTeq.into(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MinaghiAuto => MINAGHI_AUTO,
            Self::DarGlobalRealEstate => DG_REAL_ESTATE,
            Self::SalesTeq => SALESTEQ,
        }
    }
}

pub const ALL: &[InventoryType] = &[
    InventoryType::MinaghiAuto,
    InventoryType::DarGlobalRealEstate,
    InventoryType::SalesTeq,
];

pub const MINAGHI_AUTO: &'static str = "minaghi-auto";
pub const DG_REAL_ESTATE: &'static str = "dg-real-estate";
pub const SALESTEQ: &'static str = "salesteq";
