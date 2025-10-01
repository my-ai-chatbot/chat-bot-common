use std::fmt::Display;

use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(String);

impl ObjectId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn is_ok(&self) -> bool {
        for c in self.0.chars() {
            if !is_ok_char(c) {
                return false;
            }
        }

        true
    }

    pub fn fix_me(&mut self) {
        if id_would_be_ok(&self.0) {
            return;
        }

        let mut result = String::new();

        for c in self.0.chars() {
            if is_ok_char(c) {
                result.push(c);
            }
        }

        self.0 = result
    }

    pub fn from_str(value: &str) -> Option<Self> {
        if id_would_be_ok(value) {
            return Some(Self(value.to_string()));
        }
        None
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self(Default::default())
    }
}

fn id_would_be_ok(value: &str) -> bool {
    for c in value.chars() {
        if !is_ok_char(c) {
            return false;
        }
    }

    true
}
fn is_ok_char(c: char) -> bool {
    if c.is_ascii_alphabetic() {
        return true;
    }

    if c.is_ascii_digit() {
        return true;
    }

    if c == ' ' || c == '-' {
        return true;
    }

    return c as u8 >= 32;
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for ObjectId {
    fn into(self) -> String {
        self.0
    }
}

impl Into<ObjectId> for String {
    fn into(self) -> ObjectId {
        ObjectId::new(self)
    }
}

impl Into<ObjectId> for &'_ str {
    fn into(self) -> ObjectId {
        ObjectId::new(self.to_string())
    }
}

impl Into<ObjectId> for &'_ String {
    fn into(self) -> ObjectId {
        ObjectId::new(self.to_string())
    }
}

#[cfg(test)]
mod test {
    use serde::*;

    use crate::ObjectId;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Object {
        pub id: ObjectId,
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
