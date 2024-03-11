use pyramus::input::InputEvent;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

#[wasm_bindgen(js_name = inputMouseDown)]
pub fn input_mouse_down(x: f32, y: f32) -> Result<(), JsError> {
    crate::editor::input(InputEvent::MouseDown { x, y })?;
    Ok(())
}

#[wasm_bindgen(js_name = inputMouseUp)]
pub fn input_mouse_up() -> Result<(), JsError> {
    crate::editor::input(InputEvent::MouseUp)?;
    Ok(())
}

#[wasm_bindgen(js_name = inputMouseMove)]
pub fn input_mouse_move(delta_x: f32, delta_y: f32) -> Result<(), JsError> {
    crate::editor::input(InputEvent::MouseMove { delta_x, delta_y })?;
    Ok(())
}
