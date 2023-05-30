use tauri::Window;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub fn send_message(window: Window, topic: &str, message: &str) {
    println!("Sending message: {}", message);
    window
        .emit(
            topic,
            Payload {
                message: message.into(),
            },
        )
        .unwrap();
}
