// https://github.com/madonoharu/tsify/issues/42 TODO: Tsify might be a dead project, consider alternatives
#![allow(non_snake_case)]

use std::collections::HashMap;

use pyramus::models::{
    editor::{item::StageItem, staging::Staging},
    templates::prop_item::PropItemType,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrontendStage {
    pub items: HashMap<u32, FrontendItem>,
    pub selected: Vec<u32>,
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FrontendItem {
    pub id: u32,

    pub name: String,

    pub is_root: bool,

    pub parent: Option<u32>,
    pub children: Vec<u32>,

    pub item_type: FrontendItemType,

    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub rotation: f32, // In degrees
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
pub enum FrontendItemType {
    // TODO: This enum might need to be split for different stageable items
    Prop,

    Text {
        text: String,
        font_family: String,
        font_size: f32,
        color: (u8, u8, u8),
        italic: bool,
    },
    Image,
}

// Don't use 'From' trait because we want to convert with a reference, and with the stage context
impl FrontendStage {
    pub fn from(stage: &pyramus::models::editor::stage::Stage) -> FrontendStage {
        FrontendStage {
            items: stage
                .base
                .get_items()
                .iter()
                .map(|(id, item)| (id.0, FrontendItem::from(item, stage)))
                .collect::<HashMap<_, _>>(),
            selected: stage.selection.iter().map(|id| id.0).collect(),
        }
    }
}

impl FrontendItem {
    pub fn from(item: &StageItem, stage: &pyramus::models::editor::stage::Stage) -> FrontendItem {
        match item {
            StageItem::PropItem(item) => FrontendItem {
                id: item.id.0,
                name: item.name.clone(),
                is_root: item.id == stage.base.get_root(),
                parent: item.get_parent().map(|id| id.0),
                children: item.get_children().iter().map(|id| id.0).collect(),
                position: item.get_relative_transform().position,
                scale: item.get_relative_transform().scale,
                rotation: item.get_relative_transform().rotation,

                item_type: FrontendItemType::from(&item.item),
            },
            StageItem::Prop(prop) => FrontendItem {
                id: prop.id.0,
                name: prop.name.clone(),
                is_root: prop.id == stage.base.get_root(),
                parent: prop.get_parent().map(|id| id.0),
                children: prop.get_children().iter().map(|id| id.0).collect(),
                position: item.get_relative_transform().position,
                scale: item.get_relative_transform().scale,
                rotation: item.get_relative_transform().rotation,

                item_type: FrontendItemType::Prop,
            },
        }
    }
}

// Don't use 'From' trait because we want to convert with a reference
impl FrontendItemType {
    pub fn from(item_type: &PropItemType) -> FrontendItemType {
        match item_type {
            PropItemType::Text(text) => FrontendItemType::Text {
                text: text.text.clone(),
                font_family: text.font_family.clone(),
                font_size: text.font_size.get(),
                color: text.color,
                italic: text.italic,
            },
            PropItemType::Image { .. } => FrontendItemType::Image,
        }
    }
}
