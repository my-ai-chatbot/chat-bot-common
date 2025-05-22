use crate::{browsers::*, languages::*};

pub const LANG_PREFIX: &'static str = "lang:";
pub const BROWSER_PREFIX: &'static str = "browser:";

pub struct RequestMetadata {
    pub lang: Language,
    pub browser: Browser,
}

impl RequestMetadata {
    pub fn new<'s>(metadata: impl Iterator<Item = &'s str>) -> Self {
        let mut lang = Language::En;
        let mut browser = Browser::Unknown;

        for ctx in metadata {
            if ctx.starts_with(LANG_PREFIX) {
                lang = Language::from_str(&ctx[LANG_PREFIX.len() + 1..])
            }

            if ctx.starts_with(BROWSER_PREFIX) {
                browser = Browser::from_str(&ctx[BROWSER_PREFIX.len() + 1..])
            }
        }

        RequestMetadata { lang, browser }
    }

    pub fn to_metadata(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(2);

        result.push(format!("{}{}", LANG_PREFIX, self.lang.as_str()));

        if let Some(browser) = self.browser.as_str() {
            result.push(format!("{}{}", BROWSER_PREFIX, browser));
        }

        result
    }
}
