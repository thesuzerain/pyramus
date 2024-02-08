use pyramus::models::example_stage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = testRenderResvg)]
pub fn test_render(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsError> {
    Ok(pyramus::render::render(&example_stage()?, canvas)?)
}

#[wasm_bindgen(js_name = testRenderStringResvg)]
pub fn test_render_string() -> Result<String, JsError> {
    Ok(pyramus::render::render_string(&example_stage()?)?)
}
