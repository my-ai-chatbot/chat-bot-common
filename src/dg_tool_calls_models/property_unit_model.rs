use serde::*;

use crate::CurrencyConverter;

use super::*;
#[derive(Serialize, Deserialize, Clone)]
pub struct PropertyUnitJsonModel {
    pub usage_type: String,
    pub unit_type: String,
    pub unit_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_in_usd: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_year: Option<u16>,

    pub project_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apartment_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sq_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sq_feet: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub balcony_area_sq_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balcony_area_sq_feet: Option<f64>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // pub unit_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saleable_area_sq_meeters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saleable_area_sq_feet: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_latitude: Option<f64>,

    pub project_currency: String,
    pub country: String,
    pub city: String,
    //    #[serde(skip_serializing_if = "Option::is_none")]
    //    pub plot_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_slots: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_bedrooms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_of_bathrooms: Option<u32>,

    pub project: ProjectJsonModel,
}

impl PropertyUnitJsonModel {
    pub fn filter_by_price(
        &self,
        min_price_usd: Option<f64>,
        max_price_usd: Option<f64>,
        currency_converter: &impl CurrencyConverter,
    ) -> bool {
        let unit_price_usd =
            currency_converter.convert_to_usd(self.unit_price, &self.project_currency);

        if unit_price_usd.is_none() {
            panic!(
                "Unknown currency {} for property {}",
                self.project_currency, self.project_name
            );
        }

        let unit_price_usd = unit_price_usd.unwrap();

        if let Some(min_price) = min_price_usd {
            if let Some(max_price_usd) = max_price_usd {
                return min_price <= unit_price_usd && unit_price_usd <= max_price_usd;
            }
            return min_price <= unit_price_usd;
        }

        if let Some(max_price_usd) = max_price_usd {
            return unit_price_usd <= max_price_usd;
        }
        false
    }

    pub fn filter_by_city(&self, city_to_search_lower_case: &str) -> bool {
        return self.city.to_lowercase().contains(city_to_search_lower_case);
    }

    pub fn filter_by_anticipated_year(&self, year: u16) -> bool {
        let Some(expected_completion_year) = self.expected_completion_year else {
            return false;
        };

        return expected_completion_year == year;
    }

    pub fn filter_by_country(&self, country_code: &str) -> bool {
        self.country == country_code
    }
}
