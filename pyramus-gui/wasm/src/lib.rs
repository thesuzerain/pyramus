use new_editor::{CALLBACKS, RUNTIME};
use std::panic;
use wasm_bindgen::prelude::*;

pub mod create_image;
pub mod editor;
pub mod image;
pub mod input;
pub mod item;
pub mod models;
pub mod new_editor;
pub mod render; // TODO: rename

#[wasm_bindgen(start)]
pub fn init_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = Some(new_editor::Runtime::new());
    });
}

#[wasm_bindgen(js_name = subscribeFrontendCommand)]
pub fn subscribe(event: String, callback: js_sys::Function) {
    CALLBACKS.with(|callbacks| {
        let mut callbacks = callbacks.borrow_mut();
        callbacks.insert(event, callback);
    });
}

#[wasm_bindgen(js_name = testString)]
pub fn test_string() -> String {
    "Hello from Rust!".to_string()
}

/// Provides a handle to access the raw WASM memory
#[wasm_bindgen(js_name = wasmMemory)]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
