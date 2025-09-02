use serde::*;

pub const MINAGHI_AUTO: &'static str = "minaghi-auto";
pub const DG_REAL_ESTATE: &'static str = "dg-real-estate";
pub const SALESTEQ: &'static str = "salesteq";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InventoryType {
    MinaghiAuto,
    DarGlobalRealEstate,
    SalesTeq,
}

impl InventoryType {
    pub const ALL: &[InventoryType] = &[
        InventoryType::DarGlobalRealEstate,
        InventoryType::SalesTeq,
        InventoryType::MinaghiAuto,
    ];

    pub fn try_from_str(src: &str) -> Option<Self> {
        match src {
            MINAGHI_AUTO => Self::MinaghiAuto.into(),
            DG_REAL_ESTATE => Self::DarGlobalRealEstate.into(),
            SALESTEQ => Self::SalesTeq.into(),
            _ => None,
        }
    }

    pub fn from_str(src: &str) -> Self {
        Self::try_from_str(src).unwrap_or_default()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MinaghiAuto => MINAGHI_AUTO,
            Self::DarGlobalRealEstate => DG_REAL_ESTATE,
            Self::SalesTeq => SALESTEQ,
        }
    }

    pub fn is_salesteq(&self) -> bool {
        matches!(self, Self::SalesTeq)
    }

    pub fn is_minaghi_auto(&self) -> bool {
        matches!(self, Self::MinaghiAuto)
    }

    pub fn is_dar_global_real_estate(&self) -> bool {
        matches!(self, Self::DarGlobalRealEstate)
    }
}

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
        Self::ALL.into_iter().copied()
    }
}

impl crate::ItemAsStr for InventoryType {
    fn try_from_str(src: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::try_from_str(src)
    }

    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl Default for InventoryType {
    fn default() -> Self {
        InventoryType::DarGlobalRealEstate
    }
}
