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
    result.insert("terms_title", "By choosing the mode you agree with our");
    result.insert("terms_link", "Terms and Conditions");

    // Shortcut
    result.insert("auto_shortcut_leading_title", "Welcome");
    result.insert("auto_shortcut_leading_subtitle", "How can we help?");
    result.insert("auto_shortcut_caption_card_subtitle", "View");
    result.insert("auto_shortcut_caption_card_title", "Offer");
    result.insert(
        "auto_shortcut_first_sub_card_title",
        "Book a service appointment",
    );
    result.insert(
        "auto_shortcut_second_sub_card_title",
        "Request a test drive",
    );
    result.insert(
        "auto_shortcut_socials_card_title",
        "If you prefer you can continue the conversation in",
    );
    result.insert("auto_shortcut_socials_card_first_item", "What's App");
    result.insert("auto_shortcut_socials_card_second_item", "Telegram");

    // Showroom
    result.insert("auto_showroom_street", "Street");
    result.insert("auto_showroom_district", "District");
    result.insert("auto_showroom_city", "City");
    result.insert("auto_showroom_postal_code", "Postal Code");

    // Finance options
    result.insert("auto_finance_term", "Term");
    result.insert("auto_finance_interest", "Interest");
    result.insert("auto_finance_amount", "Financing amount");
    result.insert("auto_finance_balloon", "Balloon");

    // Property
    // Appointment
    result.insert("property_appointment_title", "Appointment details");
    result.insert("property_appointment_email_title", "Email");
    result.insert("property_appointment_date_title", "Date");
    result.insert("property_appointment_time_title", "Time");

    // Office
    result.insert("property_office_address_title", "Address");
    result.insert("property_office_phone_title", "Phone");

    // Shortcut
    result.insert(
        "property_shortcut_leading_title",
        "Luxury access awaits at DarGlobal.",
    );
    result.insert("property_shortcut_subtitle", "Tap to start.");
    result.insert("property_action_first_title", "Explore Properties");
    result.insert("property_action_first_prompt", "Return the latest project that was launched by DarGlobal in line with the users preferences if those are available.");
    result.insert("property_action_second_title", "Learn About Buying");
    result.insert("property_action_second_prompt", "Return a brief overview of investment potential and trust-building assets (e.g., ROI, investor testimonials, risk disclosures) for Dar Global properties.");
    result.insert("property_action_third_title", "Book a Tour or Consult");
    result.insert("property_action_third_prompt", "Return options for scheduling an efficient virtual tour or consultation, focusing on specific Dar Global projects and user availability via text-based scheduling.");

    result
}
