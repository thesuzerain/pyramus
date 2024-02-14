// https://github.com/madonoharu/tsify/issues/42 TODO: Tsify might be a dead project, consider alternatives
#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrontendStage {
    pub items: HashMap<u32, FrontendItem>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrontendItem {
    pub id: u32,

    pub name: String,

    pub parent: Option<u32>,
    pub children: Vec<u32>,

    pub item_type: FrontendItemType,

    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub rotation: f32, // In degrees
}

#[derive(Tsify, Serialize, Deserialize)]
pub enum FrontendItemType {
    Text {
        text: String,
        font_family: String,
        font_size: f32,
        color: (u8, u8, u8),
        italic: bool,
    },
    Image,
}

// Don't use 'From' trait because we want to convert with a reference
impl FrontendStage {
    pub fn from(stage: &pyramus::models::stage::Stage) -> FrontendStage {
        FrontendStage {
            items: stage
                .items
                .iter()
                .map(|(id, item)| (id.0, FrontendItem::from(item)))
                .collect::<HashMap<_, _>>(),
        }
    }
}

// Don't use 'From' trait because we want to convert with a reference
impl FrontendItem {
    pub fn from(item: &pyramus::models::item::StagedItem) -> FrontendItem {
        FrontendItem {
            id: item.id.0,
            name: item.name.clone(),
            parent: item.parent.map(|id| id.0),
            children: item.children.iter().map(|id| id.0).collect(),
            item_type: FrontendItemType::from(&item.item),
            position: item.transform.position,
            scale: item.transform.scale,
            rotation: item.transform.rotation,
        }
    }
}

// Don't use 'From' trait because we want to convert with a reference
impl FrontendItemType {
    pub fn from(item_type: &pyramus::models::item::Item) -> FrontendItemType {
        match item_type {
            pyramus::models::item::Item::Text(text) => FrontendItemType::Text {
                text: text.text.clone(),
                font_family: text.font_family.clone(),
                font_size: text.font_size.get(),
                color: text.color,
                italic: text.italic,
            },
            pyramus::models::item::Item::Image { .. } => FrontendItemType::Image,
        }
    }
}
