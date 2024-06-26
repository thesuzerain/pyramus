use editor::{dispatch_frontend_command, CALLBACKS, RUNTIME};
use pyramus::{command::FrontendCommand, models::templates::ids::PyramusId};
use std::{panic, str::FromStr};
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
pub async fn init_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let new_runtime = editor::Runtime::start().await;
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = Some(new_runtime);
    })
}

#[wasm_bindgen(js_name = switchEditorBase)]
pub fn switch_editor_base(id : String) -> Result<(), JsValue> {
    let id : PyramusId = PyramusId::from_str(&id).map_err(|e| JsValue::from_str(&format!("Could not decode PyramusId {}", e)))?;
    editor::RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let runtime = runtime.as_mut().unwrap();
        runtime.set_base(id)?;
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

#[wasm_bindgen(js_name = getStageJson)]
pub fn get_stage_json() -> Result<String, JsValue> {
    RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        let runtime = runtime
            .as_ref()
            .ok_or_else(|| JsValue::from_str("No runtime found"))?;
        let base = &runtime.stage.base;
        // TODO: This creates some huge JSON strings for images, so we need to cache those somehow.
        // (Perhaps online- or in browser cache?)
        let stage_json =
            serde_json::to_string(base).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(stage_json)
    })
}
