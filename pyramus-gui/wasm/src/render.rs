use wasm_bindgen::prelude::*;

use crate::editor;

#[wasm_bindgen(js_name = testRenderStringResvg)]
pub fn test_render_string() -> Result<String, JsError> {
    editor::RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        runtime
            .as_ref()
            .map(|runtime| runtime.render_string())
            .ok_or_else(|| pyramus::PyramusError::NoRuntimeFound)?
    })
}