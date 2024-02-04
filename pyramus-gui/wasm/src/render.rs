use pyramus::models::{example_stage, Stage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = testRenderResvg)]
pub fn test_render(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    pyramus::render::render(&example_stage(), canvas)
}

#[wasm_bindgen(js_name = testRenderStringResvg)]
pub fn test_render_string(canvas: &web_sys::HtmlCanvasElement) -> Result<String, JsValue> {
    pyramus::render::render_string(&example_stage(), canvas)
}