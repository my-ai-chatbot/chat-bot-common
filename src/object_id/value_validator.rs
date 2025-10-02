pub trait ValueValidator {
    fn validate_value(&self) -> Option<bool>;
}

pub trait IdExtension {
    fn validate_value(src: &str) -> Option<bool>;
}

impl ValueValidator for String {
    fn validate_value(&self) -> Option<bool> {
        None
    }
}

impl ValueValidator for &'_ str {
    fn validate_value(&self) -> Option<bool> {
        None
    }
}

impl ValueValidator for &'_ String {
    fn validate_value(&self) -> Option<bool> {
        None
    }
}
