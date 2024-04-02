use serde::{Deserialize, Serialize};

use crate::models::templates::{
    ids::ItemId, prop::Prop, prop_item::PropItem, transform::RelativeTransform,
};

use super::item::StageItem;

/// A trait for items that can be placed on the stage.
/// TODO: We may want to take some of the BaseItem functions and put them here
pub trait Staging {
    fn get_parent(&self) -> Option<ItemId>;
    fn set_parent(&mut self, parent: Option<ItemId>);
    fn get_children(&self) -> &Vec<ItemId>;
    fn get_children_mut(&mut self) -> &mut Vec<ItemId>;
    fn get_relative_transform(&self) -> &RelativeTransform;
    fn get_relative_transform_mut(&mut self) -> &mut RelativeTransform;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingContext {
    pub parent: Option<ItemId>,
    pub children: Vec<ItemId>,
    pub transform: RelativeTransform,
}

impl Default for StagingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl StagingContext {
    pub fn new() -> StagingContext {
        StagingContext {
            parent: None,
            children: vec![],
            transform: RelativeTransform::default(),
        }
    }
}

// TODO: Macro to simplify?
impl Staging for StageItem {
    fn get_parent(&self) -> Option<ItemId> {
        match self {
            StageItem::PropItem(item) => item.get_parent(),
            StageItem::Prop(prop) => prop.get_parent(),
        }
    }

    fn set_parent(&mut self, parent: Option<ItemId>) {
        match self {
            StageItem::PropItem(item) => item.set_parent(parent),
            StageItem::Prop(prop) => prop.set_parent(parent),
        }
    }

    fn get_children(&self) -> &Vec<ItemId> {
        match self {
            StageItem::PropItem(item) => item.get_children(),
            StageItem::Prop(prop) => prop.get_children(),
        }
    }

    fn get_children_mut(&mut self) -> &mut Vec<ItemId> {
        match self {
            StageItem::PropItem(item) => item.get_children_mut(),
            StageItem::Prop(prop) => prop.get_children_mut(),
        }
    }

    fn get_relative_transform(&self) -> &RelativeTransform {
        match self {
            StageItem::PropItem(item) => item.get_relative_transform(),
            StageItem::Prop(prop) => prop.get_relative_transform(),
        }
    }

    fn get_relative_transform_mut(&mut self) -> &mut RelativeTransform {
        match self {
            StageItem::PropItem(item) => item.get_relative_transform_mut(),
            StageItem::Prop(prop) => prop.get_relative_transform_mut(),
        }
    }
}

impl Staging for Prop {
    fn get_parent(&self) -> Option<ItemId> {
        self.staging.parent
    }

    fn set_parent(&mut self, parent: Option<ItemId>) {
        self.staging.parent = parent;
    }

    fn get_children(&self) -> &Vec<ItemId> {
        &self.staging.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<ItemId> {
        &mut self.staging.children
    }

    fn get_relative_transform(&self) -> &RelativeTransform {
        &self.staging.transform
    }

    fn get_relative_transform_mut(&mut self) -> &mut RelativeTransform {
        &mut self.staging.transform
    }
}

impl Staging for PropItem {
    fn get_parent(&self) -> Option<ItemId> {
        self.staging.parent
    }

    fn set_parent(&mut self, parent: Option<ItemId>) {
        self.staging.parent = parent;
    }

    fn get_children(&self) -> &Vec<ItemId> {
        &self.staging.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<ItemId> {
        &mut self.staging.children
    }

    fn get_relative_transform(&self) -> &RelativeTransform {
        &self.staging.transform
    }

    fn get_relative_transform_mut(&mut self) -> &mut RelativeTransform {
        &mut self.staging.transform
    }
}
