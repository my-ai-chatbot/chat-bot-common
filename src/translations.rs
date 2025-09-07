use std::collections::BTreeMap;

use crate::inventory_type::InventoryType;

impl InventoryType {
    pub fn get_en_translation(&self) -> BTreeMap<&'static str, &'static str> {
        match self {
            InventoryType::MinaghiAuto => fill_minaghi_en(),
            InventoryType::DarGlobalRealEstate => get_dg_en(),
            InventoryType::SalesTeq => get_dg_en(),
        }
    }
}

fn get_dg_en() -> BTreeMap<&'static str, &'static str> {
    let mut result: BTreeMap<&'static str, &'static str> = BTreeMap::new();
    result.insert(
        "greeting_title",
        "I'm Sarah, let me help you how to 5x your return in 2 years",
    );
    result.insert("input_placeholder", "Ask Sarah");
    result.insert("btn_send", "Send");
    result.insert("is_thinking", "Sarah is thinking...");
    result.insert("is_speaking", "Sarah is speaking...");
    result.insert("end_chat", "End chat");
    result.insert("text_chat_btn_title", "Text Chat");
    result.insert("voice_chat_btn_title", "Voice chat");

    result.insert("header_voice_chat_btn_title", "Voice Chat");
    result.insert("widget_folded_message", "5x Return in 2 Years");

    result
}

fn fill_minaghi_en() -> BTreeMap<&'static str, &'static str> {
    let mut result: BTreeMap<&'static str, &'static str> = BTreeMap::new();

    result.insert(
        "greeting_title",
        "I'm Sarah, let me help you how to 5x your return in 2 years",
    );
    result.insert("input_placeholder", "Ask Sarah");
    result.insert("btn_send", "Send");
    result.insert("is_thinking", "Sarah is thinking...");
    result.insert("is_speaking", "Sarah is speaking...");
    result.insert("end_chat", "End chat");
    result.insert("text_chat_btn_title", "Text Chat");
    result.insert("voice_chat_btn_title", "Voice Chat");
    result.insert("header_voice_chat_btn_title", "Voice Chat");
    result.insert("widget_folded_message", "5x Return in 2 Years");

    result.insert("auto_finance_amount", "Financing amount");
    result.insert("auto_finance_balloon", "Balloon");
    result.insert("auto_finance_interest", "Interest");
    result.insert("auto_finance_term", "Term");

    result.insert("auto_shortcut_caption_card_subtitle", "View");
    result.insert("auto_shortcut_caption_card_title", "Offer");
    result.insert(
        "auto_shortcut_first_sub_card_title",
        "Book a service appointment",
    );
    result.insert("auto_shortcut_leading_subtitle", "How can we help?");
    result.insert("auto_shortcut_leading_title", "Welcome");
    result.insert(
        "auto_shortcut_second_sub_card_title",
        "Request a test drive",
    );
    result.insert("auto_shortcut_socials_card_first_item", "What's App");
    result.insert("auto_shortcut_socials_card_second_item", "Telegram");
    result.insert(
        "auto_shortcut_socials_card_title",
        "If you prefer you can continue the conversation in",
    );

    result.insert("auto_showroom_city", "City");
    result.insert("auto_showroom_district", "District");
    result.insert("auto_showroom_postal_code", "Postal Code");
    result.insert("auto_showroom_street", "Street");

    result.insert("terms_link", "Terms and Conditions");
    result.insert("terms_title", "By choosing the mode you agree with our");

    result
}
