use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PropertyUnitJsonModel {
    pub project_id: String,
    pub usage_type: String,
    pub unit_type: String,
    pub unit_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_in_usd: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
}

impl PropertyUnitJsonModel {
    /*
        pub fn to_result_model(&self, project: &ProjectJsonModel) -> PropertyToolCallResponseJsonModel {
            PropertyToolCallResponseJsonModel {
                id: project.id.clone(),
                title: project.title.clone(),
                rich_description: project.rich_description.clone(),
                description: project.description.clone(),
                project_type: project.project_type.clone(),
                amenities: project.amenities.clone(),
                usage_type: self.usage_type.clone(),
                unit_type: self.unit_type.clone(),
                unit_price: self.unit_price.clone(),
                unit_price_in_usd: self.unit_price_in_usd.clone(),
                expected_completion_date: self.expected_completion_date.clone(),
                start_date: self.start_date.clone(),
                apartment_type: self.apartment_type.clone(),
                area_sq_meters: self.area_sq_meters.clone(),
                area_sq_feet: self.area_sq_feet.clone(),
                balcony_area_sq_meters: self.balcony_area_sq_meters.clone(),
                balcony_area_sq_feet: self.balcony_area_sq_feet.clone(),
                saleable_area_sq_meeters: self.saleable_area_sq_meeters.clone(),
                saleable_area_sq_feet: self.saleable_area_sq_feet.clone(),
                property_status: self.property_status.clone(),
                project_longitude: self.project_longitude.clone(),
                project_latitude: self.project_latitude.clone(),
                project_currency: self.project_currency.clone(),
                country: self.country.clone(),
                city: self.city.clone(),
                parking_slots: self.parking_slots.clone(),
                no_of_bedrooms: self.no_of_bedrooms.clone(),
                no_of_bathrooms: self.no_of_bathrooms.clone(),
                unit_category: self.unit_category.clone(),
                floor: self.floor.clone(),
                why_invest: project.why_invest.clone(),
            }
        }
    */
    pub fn filter_by_price(&self, min_price_usd: Option<f64>, max_price_usd: Option<f64>) -> bool {
        if self.unit_price_in_usd.is_none() {
            println!(
                "Unknown currency {} for property {}",
                self.project_currency, self.project_name
            );
            return false;
        }

        let unit_price_usd = self.unit_price_in_usd.unwrap();

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

    pub fn filter_by_country(&self, country_code: &str) -> bool {
        self.country == country_code
    }
}
