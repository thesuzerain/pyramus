use std::sync::Arc;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct StagedItemId(u64);

pub struct Stage {
    pub size: (u32, u32),
    pub items : Vec<StagedItem>,
}
impl Stage {
    pub fn new() -> Stage {
        Stage {
            size: (800, 600),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item, parent: Option<StagedItemId>) -> StagedItemId {
        let id = StagedItemId(self.items.len() as u64);
        self.items.push(StagedItem::new(id, item, parent));
        id
    }
}

pub struct StagedItem {
    pub id: StagedItemId,
    pub item: Item,
    
    pub parent: Option<StagedItemId>, // If None, then it's a root item
    pub relative_position: (f32, f32),
    pub relative_scale: (f32, f32),
    pub relative_rotation: f32, // In degrees
}

impl StagedItem {
    pub fn new(id : StagedItemId, item: Item, parent: Option<StagedItemId>) -> StagedItem {
        // TODO: Randomly generate id
        StagedItem {
            id,
            item,
            parent,
            relative_position: (0.0, 0.0),
            relative_scale: (1.0, 1.0),
            relative_rotation: 0.0,
        }
    }
}

pub enum Item {
    Text(ItemText),
    Image(ItemImage),
    // SvgPath``
}

pub struct ItemText {
    pub text: String,
}


pub enum ItemImage {
    Png(Arc<Vec<u8>>), 
    Jpeg(Arc<Vec<u8>>),
    Gif(Arc<Vec<u8>>),
    // Svg (should be Tree)
}


// TODO: Remove
pub fn example_stage() -> Stage {
    let mut stage = Stage::new();
    let root_id = stage.add_item(Item::Text(ItemText { text: "Hello, world!".to_string() }), None);
    stage.add_item(Item::Image(ItemImage::Jpeg( 
        include_bytes!("../../testimg.jpg").to_vec().into()
    )), None);
    stage
}

