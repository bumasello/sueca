use web_sys::window;

pub fn set_token(token: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        storage.set_item("session_id", token).ok();
    }
}

pub fn get_token() -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item("session_id").ok())
        .flatten()
}
