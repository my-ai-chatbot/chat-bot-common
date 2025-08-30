#[derive(Debug, Clone, Copy)]
pub enum InventoryType {
    MinaghiAuto,
    DarGlobalRealEstate,
    SalesTeq,
}

impl InventoryType {
    pub fn try_from_str(src: &str) -> Option<Self> {
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
    InventoryType::DarGlobalRealEstate,
    InventoryType::SalesTeq,
    InventoryType::MinaghiAuto,
];

pub const MINAGHI_AUTO: &'static str = "minaghi-auto";
pub const DG_REAL_ESTATE: &'static str = "dg-real-estate";
pub const SALESTEQ: &'static str = "salesteq";

impl crate::EnumIterator for InventoryType {
    type TItem = Self;

    fn get_value(&self) -> Self
    where
        Self: Sized,
    {
        *self
    }

    fn get_all(&self) -> impl Iterator<Item = Self>
    where
        Self: Sized,
    {
        ALL.into_iter().copied()
    }
}

impl crate::ItemAsStr for InventoryType {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}
