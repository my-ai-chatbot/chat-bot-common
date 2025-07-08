use serde::*;

use crate::{browsers::*, languages::*};

pub const LANG_PREFIX: &'static str = "lang:";
pub const BROWSER_PREFIX: &'static str = "browser:";
pub const TENANT_PREFIX: &'static str = "tenant:";
pub const SESSION_PREFIX: &'static str = "session:";
pub const TIMEZONE_PREFIX: &'static str = "tz:";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetadata {
    pub lang: Language,
    pub browser: Browser,
    pub tenant: String,
    pub session_id: String,
    pub time_zone: String,
}

impl RequestMetadata {
    pub fn new<'s>(metadata: impl Iterator<Item = &'s str>) -> Self {
        let mut lang = Language::En;
        let mut browser = Browser::Unknown;
        let mut tenant = None;
        let mut session = None;
        let mut time_zone = None;

        for ctx in metadata {
            if ctx.starts_with(LANG_PREFIX) {
                lang = Language::from_str(&ctx[LANG_PREFIX.len()..])
            }

            if ctx.starts_with(BROWSER_PREFIX) {
                browser = Browser::from_str(&ctx[BROWSER_PREFIX.len()..])
            }

            if ctx.starts_with(TENANT_PREFIX) {
                let value = &ctx[TENANT_PREFIX.len()..];
                tenant = Some(value.to_string());
            }

            if ctx.starts_with(SESSION_PREFIX) {
                let value = &ctx[SESSION_PREFIX.len()..];
                session = Some(value.to_string());
            }

            if ctx.starts_with(TIMEZONE_PREFIX) {
                let value = &ctx[TIMEZONE_PREFIX.len()..];
                time_zone = Some(value.to_string());
            }
        }

        if tenant.is_none() {
            println!("Can not extract tenant from metadata");
            panic!("Can not extract tenant from metadata");
        }

        if session.is_none() {
            println!("Can not extract session from metadata");
            panic!("Can not extract session from metadata");
        }

        if time_zone.is_none() {
            println!("Can not extract timezone from metadata");
            panic!("Can not extract timezone from metadata");
        }

        RequestMetadata {
            lang,
            browser,
            tenant: tenant.unwrap(),
            session_id: session.unwrap(),
            time_zone: time_zone.unwrap(),
        }
    }

    pub fn to_metadata(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(3);

        result.push(format!("{}{}", TENANT_PREFIX, self.tenant.as_str()));
        result.push(format!("{}{}", SESSION_PREFIX, self.session_id.as_str()));

        result.push(format!("{}{}", LANG_PREFIX, self.lang.as_str()));
        result.push(format!("{}{}", TIMEZONE_PREFIX, self.time_zone));

        if let Some(browser) = self.browser.as_str() {
            result.push(format!("{}{}", BROWSER_PREFIX, browser));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::RequestMetadata;

    #[test]
    fn test_parsing_metadata() {
        let ctx = vec!["lang:ar"];

        let meta_data = RequestMetadata::new(ctx.into_iter());

        assert_eq!("ar", meta_data.lang.as_str())
    }
}
