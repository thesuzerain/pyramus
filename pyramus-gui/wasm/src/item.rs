use crate::{
    editor::{self, command},
    models::FrontendStage,
};
use pyramus::{command::BackendCommand, models::item::StagedItemId};
use wasm_bindgen::prelude::*;

// TODO: Coalesce these into a struct

#[wasm_bindgen(js_name = removeObject)]
// TODO: should we have a way this can directly return an error?
pub fn remove_object(item_id: u32) -> Result<(), JsError> {
    command([BackendCommand::DeleteItem(StagedItemId(item_id))])?;
    Ok(())
}

#[wasm_bindgen(js_name = getStage)]
pub fn get_items() -> Result<FrontendStage, JsError> {
    editor::RUNTIME.with(|runtime| {
        let runtime = runtime.borrow();
        Ok(runtime
            .as_ref()
            .map(|runtime| FrontendStage::from(&runtime.stage))
            .ok_or_else(|| pyramus::PyramusError::NoRuntimeFound)?)
    })
}
