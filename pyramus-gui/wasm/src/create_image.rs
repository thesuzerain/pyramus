use pyramus::{
    command::BackendCommand,
    models::templates::{builder::ItemBuilder, ids::ItemId},
};
use wasm_bindgen::prelude::*;

use crate::new_editor::command;

#[wasm_bindgen(js_name = uploadImage)]
pub fn upload_image(name: String, parent: u32, data: Vec<u8>) -> Result<(), JsError> {
    command(vec![BackendCommand::CreateItem {
        new_item: ItemBuilder::build_image_from_bytes(data, "png")
            .name(name)
            .parent(ItemId(parent)), // TODO: more than just png
    }])?;
    Ok(())
}

#[wasm_bindgen(js_name = uploadSvg)]
pub fn upload_svg(name: String, parent: u32, svg: String) -> Result<(), JsError> {
    command(vec![BackendCommand::CreateItem {
        new_item: ItemBuilder::build_image_from_svg(svg)
            .name(name)
            .parent(ItemId(parent)),
    }])?;
    Ok(())
}

#[wasm_bindgen(js_name = uploadText)]
pub fn upload_text(name: String, parent: u32, text: String) -> Result<(), JsError> {
    command(vec![BackendCommand::CreateItem {
        new_item: ItemBuilder::build_text_basic(text)
            .name(name)
            .parent(ItemId(parent)),
    }])?;
    Ok(())
}
