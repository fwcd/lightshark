use async_tungstenite::tungstenite::Message;

/// Formats a WebSocket message for output.
pub(crate) fn format_ws_message(msg: &Message) -> String {
    match msg {
        Message::Text(text) => format!(r#"Text: "{}""#, text),
        Message::Ping(ping) => format!("Ping: {:?}", ping),
        Message::Pong(pong) => format!("Pong: {:?}", pong),
        Message::Close(frame) => format!("Close: {:?}", frame),
        Message::Frame(frame) => format!("Frame: {:?}", frame),
        Message::Binary(bin) => {
            match rmpv::decode::read_value(&mut &bin[..]) {
                Ok(mp) => format!("MessagePack: {}", mp),
                Err(_) => format!("Binary: {:?}", bin),
            }
        },
    }
}
