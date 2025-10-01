pub fn validate_generic_object_id(src: &str) -> Option<bool> {
    if src.len() == 0 {
        return None;
    }

    let mut first = ' ';
    let mut last = ' ';

    for (i, c) in src.chars().enumerate() {
        if !is_ok_char(c) {
            return Some(false);
        }

        if i == 0 {
            first = c;
        }

        last = c;
    }

    if first == ' ' {
        return Some(false);
    }

    if last == ' ' {
        return Some(false);
    }

    Some(true)
}

pub fn is_ok_char(c: char) -> bool {
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

pub fn generic_id_would_be_ok(value: &str) -> bool {
    for c in value.chars() {
        if !is_ok_char(c) {
            return false;
        }
    }

    true
}
