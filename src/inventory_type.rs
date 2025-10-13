use std::str::FromStr;

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

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
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

impl Default for InventoryType {
    fn default() -> Self {
        InventoryType::DarGlobalRealEstate
    }
}

#[cfg(feature = "dioxus")]
impl dioxus_admin_ui_kit::types::EnumIterator for InventoryType {
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

impl rust_extensions::AsStr for InventoryType {
    fn as_str(&self) -> &'static str {
        self.as_str()
    }
}

impl FromStr for InventoryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::try_from_str(s) {
            Some(value) => return Ok(value),
            None => return Err(format!("Invalid '{}' value to parse InventoryType", s)),
        }
    }
}
