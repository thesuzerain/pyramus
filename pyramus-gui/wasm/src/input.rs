use pyramus::input::InputEvent;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};


#[wasm_bindgen(js_name = inputClick)]
pub fn input_click(x : f32, y : f32) -> Result<(), JsError> {
    crate::editor::input(InputEvent::Click { x, y })?;
    Ok(())
}
