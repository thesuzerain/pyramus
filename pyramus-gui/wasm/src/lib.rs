use editor::{dispatch_frontend_command, CALLBACKS, RUNTIME};
use pyramus::command::FrontendCommand;
use std::panic;
use wasm_bindgen::prelude::*;

// TODO: panic handler

pub mod create_image;
pub mod editor;
pub mod image;
pub mod input;
pub mod item;
pub mod models;
pub mod render; // TODO: rename

#[wasm_bindgen(start)]
pub fn init_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = Some(editor::Runtime::new());
    });
}

#[wasm_bindgen(js_name = switchProp)]
pub fn switch_prop() -> Result<(), JsValue> {
    editor::RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let runtime = runtime.as_mut().unwrap();
        runtime.set_prop();
    });

    // TODO: Reorganize this
    CALLBACKS.with(|callbacks| {
        let js_callbacks = callbacks.borrow();
        let command = FrontendCommand::Rerender;
        dispatch_frontend_command(&js_callbacks, command)?;
        Ok::<(), JsError>(())
    })?;

    Ok(())
}

#[wasm_bindgen(js_name = switchBlueprint)]
pub fn switch_blueprint() -> Result<(), JsValue> {
    editor::RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let runtime = runtime.as_mut().unwrap();
        runtime.set_blueprint();
        Ok::<(), JsError>(())
    })?;

    // TODO: Reorganize this
    CALLBACKS.with(|callbacks| {
        let js_callbacks = callbacks.borrow();
        let command = FrontendCommand::Rerender;
        dispatch_frontend_command(&js_callbacks, command)?;
        Ok::<(), JsError>(())
    })?;

    Ok(())
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
