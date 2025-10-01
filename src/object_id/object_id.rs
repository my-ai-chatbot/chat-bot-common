use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use super::*;
use serde::*;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ObjectId<TObjectIdValidator: ValueValidator> {
    value: String,
    #[serde(skip)]
    phantom: PhantomData<TObjectIdValidator>,
}

impl<TObjectIdValidator: ValueValidator> ObjectId<TObjectIdValidator> {
    pub const DEFAULT_REF: Self = Self {
        value: String::new(),
        phantom: PhantomData,
    };

    pub fn new(value: String) -> Self {
        Self {
            value,
            phantom: Default::default(),
        }
    }

    pub fn is_ok(&self) -> Option<bool> {
        TObjectIdValidator::validate_value(&self.value)
    }

    pub fn fix_me(&mut self) {
        if super::utils::generic_id_would_be_ok(&self.value) {
            return;
        }

        let mut result = String::new();

        for c in self.value.chars() {
            if super::utils::is_ok_char(c) {
                result.push(c);
            }
        }

        self.value = result
    }

    pub fn from_str(value: &str) -> Self {
        Self::new(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn eq_with_opt(&self, other: Option<&ObjectId<TObjectIdValidator>>) -> bool {
        match other {
            Some(other) => self.eq_with(other),
            None => false,
        }
    }

    pub fn eq_with(&self, other: &ObjectId<TObjectIdValidator>) -> bool {
        self.value == other.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl<TObjectIdValidator: ValueValidator> Default for ObjectId<TObjectIdValidator> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<TObjectIdValidator: ValueValidator> AsRef<str> for ObjectId<TObjectIdValidator> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<TObjectIdValidator: ValueValidator> Display for ObjectId<TObjectIdValidator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<TObjectIdValidator: ValueValidator> Debug for ObjectId<TObjectIdValidator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<TObjectIdValidator: ValueValidator> Into<String> for ObjectId<TObjectIdValidator> {
    fn into(self) -> String {
        self.value
    }
}

impl<TObjectIdValidator: ValueValidator> Into<ObjectId<TObjectIdValidator>> for String {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self)
    }
}

impl<TObjectIdValidator: ValueValidator> Into<ObjectId<TObjectIdValidator>> for &'_ str {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self.to_string())
    }
}

impl<TObjectIdValidator: ValueValidator> Into<ObjectId<TObjectIdValidator>> for &'_ String {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self.to_string())
    }
}

#[cfg(test)]
mod test {
    use serde::*;

    use crate::object_id::ValueValidator;

    use super::ObjectId;

    pub struct TestObjectIdValidator;

    impl ValueValidator for TestObjectIdValidator {
        fn validate_value(src: &str) -> Option<bool> {
            super::super::utils::validate_generic_object_id(src)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Object {
        pub id: ObjectId<TestObjectIdValidator>,
        pub name: String,
    }

    #[test]
    fn test() {
        let object = Object {
            id: "id".into(),
            name: "name".into(),
        };

        let result = serde_json::to_string(&object).unwrap();

        println!("{}", result);
    }
}
