use std::collections::BTreeMap;

pub fn en() -> BTreeMap<&'static str, &'static str> {
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

/*
translations: {
        greeting_title: "I'm Sarah, let me help you how to 5x your return in 2 years",
        input_placeholder: 'Ask Sarah',
        btn_send: 'Send',
        is_thinking: 'Sarah is thinking...',
        is_speaking: 'Sarah is speaking...',
        end_chat: 'End chat',
        text_chat_btn_title: 'Text Chat',
        voice_chat_btn_title: 'Voice Chat',
        header_voice_chat_btn_title: 'Voice Chat',
        widget_folded_messages: [
            '5x Return in 2 Years'
        ]
    },
*/
