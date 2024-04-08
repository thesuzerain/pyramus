use super::{base_item::BaseItem, staging::Staging};
use crate::models::templates::{ids::ItemId, prop::Prop, prop_item::PropItem};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// An item that can be placed on the stage, or as part of a BaseItem
/// StageItem, as well as its variants, should implement the Staging trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageItem {
    PropItem(PropItem),
    Prop(Prop),
}

impl StageItem {
    // TODO: should we keep these as the trait functions? are these necessary alongside the trait functions?
    // TODO: These are basically 1:1 with the trait items in some cases

    /// Get the item id of the StageItem
    pub fn get_id(&self) -> ItemId {
        match self {
            StageItem::PropItem(item) => item.id,
            StageItem::Prop(prop) => prop.id,
        }
    }

    /// Rename the StageItem
    pub fn rename(&mut self, name: String) {
        match self {
            StageItem::PropItem(item) => item.name = name,
            StageItem::Prop(prop) => prop.name = name,
        }
    }

    /// Set the parent of the StageItem within its BaseItem
    pub fn set_parent(&mut self, parent: Option<ItemId>) {
        match self {
            StageItem::PropItem(item) => item.set_parent(parent),
            StageItem::Prop(prop) => prop.set_parent(parent),
        }
    }

    /// Get the local size of the StageItem
    pub fn get_size(&self) -> (u32, u32) {
        let (x0, y0, x1, x2) = match self {
            StageItem::PropItem(item) => item.get_local_bounds(),
            StageItem::Prop(prop) => prop.get_local_bounds(),
        };
        ((x1 - x0) as u32, (x2 - y0) as u32)
    }

    /// Get the local bounds of the item
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        match self {
            StageItem::PropItem(item) => item.get_local_bounds(),
            StageItem::Prop(prop) => prop.get_local_bounds(),
        }
    }

    /// Check if a point in screen space is within the bounds of the item
    pub fn contains_point(&self, x: f32, y: f32, container_item: &BaseItem) -> bool {
        // Get transform of current item
        let transform = self.get_screen_transform(container_item);

        // Get the click in local space and check if it's within the bounds of the item
        let click = transform.inverse().transform_point2(glam::Vec2::new(x, y));
        let (x0, y0, x1, y1) = self.get_local_bounds();
        click.x >= x0 && click.x <= x1 && click.y >= y0 && click.y <= y1
    }

    /// Get the bounds of the item in screen space
    /// x0, y0, x1, y1
    pub fn get_bounds(&self, container_item: &BaseItem) -> (f32, f32, f32, f32) {
        let (x0, y0, x1, y1) = self.get_local_bounds();

        let transform = self.get_screen_transform(container_item);
        let Vec2 { x: x0, y: y0 } = transform.transform_point2(glam::Vec2::new(x0, y0));
        let Vec2 { x: x1, y: y1 } = transform.transform_point2(glam::Vec2::new(x1, y1));

        (
            f32::min(x0, x1),
            f32::min(y0, y1),
            f32::max(x0, x1),
            f32::max(y0, y1),
        )
    }

    /// Get the transform of the item in screen space of a container item
    pub fn get_screen_transform(&self, container_item: &BaseItem) -> glam::Affine2 {
        // TODO: If we add 3d, this needs a projection matrix/camera and world space as an intermediate step
        let transform = self.get_relative_transform().to_glam_affine();
        if let Some(parent_id) = self.get_parent() {
            let parent_item = container_item
                .get_item(parent_id)
                .expect("Parent item not found");
            parent_item.get_screen_transform(container_item) * transform
        } else {
            transform
        }
    }
}
