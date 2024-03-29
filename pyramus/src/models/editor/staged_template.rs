use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::templates::{
    blueprint::Blueprint, builder::ItemBuilder, ids::ItemId, prop::Prop,
    transform::RelativeTransform,
};

use super::item::StageItem;

// TODO: Trait-ify this as much as you can

#[derive(Debug, Serialize, Deserialize)]
pub enum BaseItem {
    Prop(Prop),
    Blueprint(Blueprint),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTemplate {
    pub items: HashMap<ItemId, StageItem>,

    // TODO: Should this crop to the total bounds of the items?
    pub size: (u32, u32),
    pub root: ItemId,
}

impl BaseItem {
    // TODO: Trait-ify this as much as you can.
    // A lot of stuff here can be traited- even if the whole thing can't be (due to issues with BackendCommand)
    pub fn get_template_mut(&mut self) -> &mut BaseTemplate {
        match self {
            BaseItem::Prop(prop) => &mut prop.template,
            BaseItem::Blueprint(blueprint) => &mut blueprint.template,
        }
    }

    pub fn get_template(&self) -> &BaseTemplate {
        match self {
            BaseItem::Prop(prop) => &prop.template,
            BaseItem::Blueprint(blueprint) => &blueprint.template,
        }
    }

    // TODO: This pattern could be improved (taking in an Rc<RwLock> parent, rather than a reference to self)
    pub fn add_child(&mut self, item_builder: ItemBuilder) -> crate::Result<ItemId> {
        let template = self.get_template_mut();

        // TODO: Revisit this function after blueprint refactor- move to prop?
        let parent = item_builder.parent.unwrap_or(template.root);

        // Check parent exists and add child
        let parent = template
            .items
            .get_mut(&parent)
            .ok_or_else(|| crate::PyramusError::OtherError("Parent not found".to_string()))?;

        // Build and insert item
        let item = item_builder.build()?;
        let id = item.get_id();

        // Insert parent and child
        parent.get_children_mut().push(id);
        template.items.insert(id, item);

        Ok(id)
    }

    pub fn edit_item(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut StageItem) -> crate::Result<()>,
    ) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        let template = self.get_template_mut();
        if let Some(item) = template.items.get_mut(&id) {
            f(item)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn get_item(&self, id: ItemId) -> Option<&StageItem> {
        self.get_template().items.get(&id)
    }

    pub fn get_items(&self) -> &HashMap<ItemId, StageItem> {
        &self.get_template().items
    }

    pub fn get_root(&self) -> ItemId {
        self.get_template().root
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.get_template().size
    }

    pub fn edit_item_transform(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut RelativeTransform) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let template = self.get_template_mut();
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot edit the root item
        if id == template.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot edit the root item".to_string(),
            ));
        }

        if let Some(item) = template.items.get_mut(&id) {
            f(item.get_relative_transform_mut())
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn remove_item(&mut self, id: ItemId) -> crate::Result<()> {
        let template = self.get_template_mut();
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot remove the root item
        if id == template.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot remove the root item".to_string(),
            ));
        }

        let root = template.root;
        let parent = template.items.get(&id).and_then(|item| item.get_parent());
        let children = template
            .items
            .get(&id)
            .map(|item| item.get_children().clone())
            .unwrap_or_default();

        if let Some(parent) = parent.and_then(|parent| template.items.get_mut(&parent)) {
            parent.get_children_mut().retain(|child| *child != id);
        }

        // Children should be kept and re-parented to the root
        // TODO: make this optional
        // If we reparent to the root, we should also calculate the new relative transform to keep the same position
        for child in children {
            let Some(child) = template.items.get_mut(&child) else {
                continue;
            };
            child.set_parent(Some(root));
        }
        template.items.remove(&id);

        Ok(())
    }
}
