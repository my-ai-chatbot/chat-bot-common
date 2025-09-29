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

    result.insert("input_placeholder", "Message");
    result.insert("btn_send", "Send");
    result.insert("text_chat_btn_title", "Text chat");
    result.insert("voice_chat_btn_title", "Voice chat");
    result.insert("header_voice_chat_btn_title", "Voice chat");
    result.insert("box_shortcut_header_first_line", "Welcome");
    result.insert("box_shortcut_header_second_line", "How can we help?");
    result.insert("box_shortcut_first_card_subtitle", "View");
    result.insert("box_shortcut_first_card_title", "Projects");
    result.insert(
        "box_shortcut_first_card_prompt",
        "Show the latest project that was launched by DarGlobal",
    );
    result.insert("box_shortcut_second_card_title", "Learn about Buying");
    result.insert(
        "box_shortcut_second_card_prompt",
        "Can you walk me briefly through the process of buying a property with DarGlobal?",
    );
    result.insert("box_shortcut_third_card_title", "Schedule a Consultation");
    result.insert("box_shortcut_third_card_prompt", "Return options for scheduling an efficient virtual tour or consultation, focusing on specific Dar Global projects and user availability via text-based scheduling.");
    result.insert(
        "box_shortcut_socials_card_title",
        "If you prefer you can continue the conversation in",
    );
    result.insert("card_brochure_download_title", "Download");
    result.insert("card_office_address_title", "Address");
    result.insert("card_office_phone_title", "Phone");
    result.insert("card_feedback_header_title", "Appointment details");
    result.insert("card_feedback_status", "Confirmed");
    result.insert("card_feedback_email", "Email");
    result.insert("card_feedback_date", "Date");
    result.insert("card_feedback_time", "Time");
    result.insert("auto_finance_amount", "");
    result.insert("auto_finance_balloon", "");
    result.insert("auto_finance_interest", "");
    result.insert("auto_finance_term", "");
    result.insert("auto_showroom_city", "");
    result.insert("auto_showroom_district", "");
    result.insert("auto_showroom_postal_code", "");
    result.insert("auto_showroom_street", "");
    result.insert(
        "terms_and_conditions_by_choosing",
        "By choosing the mode you agree to our",
    );
    result.insert("terms_and_conditions_terms_of_use", "Terms of Use");
    result.insert("terms_and_conditions_and_confirm", "and confirm that you");
    result.insert("terms_and_conditions_have_read", "have read our");
    result.insert("terms_and_conditions_privacy_policy", "Privacy Policy");
    result.insert("terms_link", "Terms and Conditions");
    result.insert("terms_title", "By choosing the mode you agree with our");
    result.insert("terms_content", "Terms of Use content here...");
    result.insert("privacy_content", "Privacy Policy content here...");
    result.insert("promo_title", "DarGlobal AI");
    result.insert(
        "promo_description",
        "Your personal assistant for the world's most exclusive real estate",
    );
    result.insert("promo_button_text", "Get started");
    result.insert("choose_mode_title", "Choose mode");
    result.insert("choose_mode_voice_btn", "Voice chat");
    result.insert("choose_mode_text_btn", "Text chat");

    result
}

fn fill_minaghi_en() -> BTreeMap<&'static str, &'static str> {
    let mut result: BTreeMap<&'static str, &'static str> = BTreeMap::new();

    result.insert("input_placeholder", "Message");
    result.insert("btn_send", "Send");
    result.insert("text_chat_btn_title", "Text Chat");
    result.insert("voice_chat_btn_title", "Voice Chat");
    result.insert("header_voice_chat_btn_title", "Voice Chat");
    result.insert("box_shortcut_header_first_line", "Welcome");
    result.insert("box_shortcut_header_second_line", "How can we help?");
    result.insert("box_shortcut_first_card_subtitle", "View");
    result.insert("box_shortcut_first_card_title", "Offer");
    result.insert("box_shortcut_first_card_prompt", "Show me properties");
    result.insert(
        "box_shortcut_second_card_title",
        "Book a service appointment",
    );
    result.insert("box_shortcut_second_card_prompt", "Schedule viewing");
    result.insert("box_shortcut_third_card_title", "Request a test drive");
    result.insert("box_shortcut_third_card_prompt", "Get consultation");
    result.insert(
        "box_shortcut_socials_card_title",
        "If you prefer you can continue the conversation in",
    );
    result.insert("card_brochure_download_title", "Download");
    result.insert("card_office_address_title", "Address");
    result.insert("card_office_phone_title", "Phone");
    result.insert("card_feedback_header_title", "Appointment details");
    result.insert("card_feedback_status", "Confirmed");
    result.insert("card_feedback_email", "Email");
    result.insert("card_feedback_date", "Date");
    result.insert("card_feedback_time", "Time");
    result.insert("auto_finance_amount", "Financing amount");
    result.insert("auto_finance_balloon", "Balloon");
    result.insert("auto_finance_interest", "Interest");
    result.insert("auto_finance_term", "Term");
    result.insert("auto_showroom_city", "City");
    result.insert("auto_showroom_district", "District");
    result.insert("auto_showroom_postal_code", "Postal Code");
    result.insert("auto_showroom_street", "Street");
    result.insert(
        "terms_and_conditions_by_choosing",
        "By choosing the mode you agree to our",
    );
    result.insert("terms_and_conditions_terms_of_use", "Terms of Use");
    result.insert("terms_and_conditions_and_confirm", "and confirm that you");
    result.insert("terms_and_conditions_have_read", "have read our");
    result.insert("terms_and_conditions_privacy_policy", "Privacy Policy");
    result.insert("terms_link", "Terms and Conditions");
    result.insert("terms_title", "By choosing the mode you agree with our");
    result.insert("terms_content", "Terms of Use content here...");
    result.insert("privacy_content", "Privacy Policy content here...");
    result.insert("promo_title", "DarGlobal AI");
    result.insert(
        "promo_description",
        "Your personal assistant for the world's most exclusive real estate",
    );
    result.insert("promo_button_text", "Get started");
    result.insert("choose_mode_title", "Choose mode");
    result.insert("choose_mode_voice_btn", "Voice chat");
    result.insert("choose_mode_text_btn", "Text chat");
    result.insert("car_offer_label", "Most Popular");

    result.insert("card_feedback_header_title", "Appointment details");
    result.insert("card_feedback_status_success", "Confirmed");
    result.insert("card_feedback_status_canceled", "Canceled");

    result.insert("otp_resend_message", "We sent a new code over SMS to");
    result.insert(
        "otp_error_message",
        "This code doesn't look right. Please try again.",
    );

    result.insert("card_feedback_add_to_calendar", "Add to calendar");

    result.insert("otp_sent_message", "Enter the code I sent over SMS to");
    result.insert(
        "otp_header_description",
        "Enter the code I sent over SMS to",
    );

    result.insert(
        "card_feedback_text_under_card",
        "What else can I help with?",
    );

    result.insert("otp_header_title", "Phone verification");

    result.insert("otp_resend_button", "Resend code");

    result.insert("otp_update_phone_button", "Update phone");

    result.insert("otp_cancel_button", "Cancel");

    result.insert(
        "otp_success_message_in_chat",
        "Great! It's confirmed, I also sent you an SMS with this information",
    );

    result.insert(
        "otp_auth_success_in_chat_label",
        "You are now authenticated",
    );

    result.insert("car_offer_price_month", "month");
    result.insert("card_car_sales_location_address_title", "Address");
    result.insert("card_car_sales_location_phone_title", "Phone number");
    result.insert("card_car_sales_location_sales_phone_title", "Sales");
    result.insert("card_car_sales_location_service_phone_title", "Service");
    result.insert(
        "card_car_sales_location_opening_hours_title",
        "Opening hours",
    );

    result
}
