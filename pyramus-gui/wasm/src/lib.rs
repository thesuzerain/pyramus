use wasm_bindgen::prelude::*;

pub mod editor;
pub mod image;
pub mod item;
pub mod models;
pub mod render;

#[wasm_bindgen(start)]
pub fn init_app() {
    editor::RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = Some(editor::EditorRuntime::new());
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
