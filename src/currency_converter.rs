pub trait CurrencyConverter {
    fn convert_to_usd(&self, amount: f64, currency_code: &str) -> Option<f64>;
}
