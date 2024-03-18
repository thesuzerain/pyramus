use super::{ids::ItemId, prop::Prop};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blueprint {
    pub name: String,
    pub items: HashMap<ItemId, Prop>,
    pub root: ItemId,
    pub size: (u32, u32), // TODO: Should this crop to the total bounds of the items?
}

impl Blueprint {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        (0.0, 0.0, self.size.0 as f32, self.size.1 as f32)
    }
}
