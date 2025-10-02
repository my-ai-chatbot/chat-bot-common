#[derive(Debug, Clone, Copy)]
pub enum ValueValidationResult {
    Empty,
    IllegalChars,
    MinValueViolation,
    MaxValueViolation,
}

pub trait ValueValidator {
    fn validate_value(&self) -> Result<(), ValueValidationResult>;
}

impl ValueValidator for String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}

impl ValueValidator for &'_ str {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}

impl ValueValidator for str {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}

impl ValueValidator for &'_ String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}
