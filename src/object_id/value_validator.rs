pub trait ValueValidator {
    fn validate_value(&self) -> Option<bool>;
}

pub trait IdExtension {
    fn validate_value(src: &str) -> Option<bool>;
}

impl IdExtension for String {
    fn validate_value(_: &str) -> Option<bool> {
        None
    }
}

impl IdExtension for &'_ str {
    fn validate_value(_: &str) -> Option<bool> {
        None
    }
}

impl IdExtension for &'_ String {
    fn validate_value(_: &str) -> Option<bool> {
        None
    }
}
