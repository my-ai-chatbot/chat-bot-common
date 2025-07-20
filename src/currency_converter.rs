use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub struct CurrencyRate {
    pub from_currency: StrOrString<'static>,
    pub to_currency: StrOrString<'static>,
    pub rate: f64,
}

pub struct CurrencyConverter {
    data: Vec<CurrencyRate>,
}

impl CurrencyConverter {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn convert(&self, from_currency: &str, to_currency: &str, amount: f64) -> Option<f64> {
        if from_currency == to_currency {
            return Some(amount);
        }

        for itm in &self.data {
            if itm.from_currency.as_str() == from_currency
                && itm.to_currency.as_str() == to_currency
            {
                return Some(amount * itm.rate);
            }

            if itm.from_currency.as_str() == to_currency
                && itm.to_currency.as_str() == from_currency
            {
                return Some(amount / itm.rate);
            }
        }

        None
    }

    pub fn update_rate(&mut self, rate: CurrencyRate) {
        let index = self.data.iter().position(|itm| {
            itm.from_currency.as_str() == rate.from_currency.as_str()
                && itm.to_currency.as_str() == rate.to_currency.as_str()
        });

        match index {
            Some(index) => {
                self.data.get_mut(index).unwrap().rate = rate.rate;
            }
            None => {
                self.data.push(rate);
            }
        }
    }
}
