#[derive(Debug, Clone, Copy)]
pub enum ValueValidationResult {
    Empty,
    IllegalChars,
    MinValueViolation,
    MaxValueViolation,
}

pub trait ValueValidator<TValue: Sized> {
    fn validate_value(&self) -> Result<(), ValueValidationResult>;
    fn set_min_value(&mut self, value: TValue);
    fn set_max_value(&mut self, value: TValue);
}

impl ValueValidator<String> for String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
    fn set_min_value(&mut self, _value: String) {}
    fn set_max_value(&mut self, _value: String) {}
}

impl<'s> ValueValidator<&'s str> for &'s str {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }

    fn set_min_value(&mut self, _value: &'s str) {}
    fn set_max_value(&mut self, _value: &'s str) {}
}

impl<'s> ValueValidator<&'s String> for &'s String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }

    fn set_min_value(&mut self, _value: &'s String) {}
    fn set_max_value(&mut self, _value: &'s String) {}
}
