use std::collections::BTreeMap;

pub fn get_translations() -> BTreeMap<&'static str, &'static str> {
    let mut result = BTreeMap::new();
    result.insert("widget_caption", "Chat with Sarah");
    result.insert("title_header", "I'm Sarah, Your Expert in Elevated Driving");
    result.insert("input_placeholder", "Ask Sarah");
    result.insert("btn_send", "Send");
    result.insert("is_thinking", "Sarah is thinking...");
    result.insert("is_speaking", "Sarah is speaking...");
    result.insert("end_chat", "End chat");
    result.insert("voice_chat", "Voice chat");
    result
}
