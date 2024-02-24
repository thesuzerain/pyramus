use pyramus::{
    command::BackendCommand,
    models::item::{ItemBuilder, StagedItemId},
};
use wasm_bindgen::prelude::*;

use crate::editor::command;

#[wasm_bindgen(js_name = uploadImage)]
pub fn upload_image(name: String, parent: u32, data: Vec<u8>) -> Result<(), JsError> {
    command([BackendCommand::CreateItem {
        name,
        parent: StagedItemId(parent),
        new_item: ItemBuilder::ImageFromBytes {
            bytes: data,
            ext: "png".to_string(),
        },
    }])?;
    Ok(())
}

#[wasm_bindgen(js_name = uploadSvg)]
pub fn upload_svg(name: String, parent: u32, svg: String) -> Result<(), JsError> {
    command([BackendCommand::CreateItem {
        name,
        parent: StagedItemId(parent),
        new_item: ItemBuilder::ImageFromSvg(svg),
    }])?;
    Ok(())
}

#[wasm_bindgen(js_name = uploadText)]
pub fn upload_text(name: String, parent: u32, text: String) -> Result<(), JsError> {
    command([BackendCommand::CreateItem {
        name,
        parent: StagedItemId(parent),
        new_item: ItemBuilder::Text {
            text,
            // TODO: make these configurable
            font_family: "Arial".to_string(),
            font_size: 12.0,
            color: (0, 0, 0),
            italic: false,
        },
    }])?;
    Ok(())
}
