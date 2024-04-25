use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::templates::{
    blueprint::Blueprint, builder::ItemBuilder, ids::InternalId, prop::Prop,
    transform::RelativeTransform,
};

use super::{item::StageItem, staging::Staging};

// TODO: Trait-ify this as much as you can. Currently BaseItem each variant has a BaseTemplate
// TODO: Revisit how these fields are labelled- it's a little bit confusing. (item, inner hand, prop, etc). Ideally you shouldnt have to keep saying 'inner'

/// The base item that is being edited in the editor.
/// Each item has a BaseTemplate that contains the inner items and their relationships.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base {
    pub item: BaseItem,

    // TODO: Metadata fields
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BaseItem {
    Prop(Prop),
    Blueprint(Blueprint)
}

/// The base template that contains the items that make up an object.
/// Used for an item with an internal structure of items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTemplate {
    pub items: HashMap<InternalId, StageItem>,

    // TODO: Should this crop to the total bounds of the items?
    pub size: (u32, u32),
    pub root: InternalId,
}

impl Base {
    /// Create a new empty base with a base item.
    pub fn new(item: BaseItem) -> Self {
        Self {
            item,

            // TODO: Metadata fields
            author: "Unknown".to_string(),
            created_at: "Unknown".to_string(),
            updated_at: "Unknown".to_string(),
        }
    }
}

impl Base {
    // TODO: Trait-ify this as much as you can.
    // A lot of stuff here can be traited- even if the whole thing can't be (due to issues with BackendCommand)

    /// Get the inner BaseTemplate field of the item, mutably.
    pub fn get_template_mut(&mut self) -> &mut BaseTemplate {
        match &mut self.item {
            BaseItem::Prop(prop) => &mut prop.template,
            BaseItem::Blueprint(blueprint) => &mut blueprint.template,
        }
    }

    /// Get the inner BaseTemplate field of the item.
    pub fn get_template(&self) -> &BaseTemplate {
        match &self.item {
            BaseItem::Prop(prop) => &prop.template,
            BaseItem::Blueprint(blueprint) => &blueprint.template,
        }
    }

    // TODO: This pattern could be improved (taking in an Rc<RwLock> parent, rather than a reference to self)
    /// Add a child item to the inner structure of the item.
    pub fn add_child(&mut self, item_builder: ItemBuilder) -> crate::Result<InternalId> {
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

    /// Edit an inner item in the inner structure of the base item.
    pub fn edit_item(
        &mut self,
        id: InternalId,
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

    /// Get an inner item in the inner structure of the base item.
    pub fn get_item(&self, id: InternalId) -> Option<&StageItem> {
        self.get_template().items.get(&id)
    }

    /// Get the inner items in the inner structure of the base item.
    pub fn get_items(&self) -> &HashMap<InternalId, StageItem> {
        &self.get_template().items
    }

    /// Get the root item in the inner structure of the base item.
    pub fn get_root(&self) -> InternalId {
        self.get_template().root
    }

    /// Get the size of the inner structure of the base item.
    pub fn get_size(&self) -> (u32, u32) {
        self.get_template().size
    }

    /// Set the size of the inner structure of the base item.
    pub fn edit_item_transform(
        &mut self,
        id: InternalId,
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

    /// Remove an inner item in the inner structure of the base item.
    pub fn remove_item(&mut self, id: InternalId) -> crate::Result<()> {
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


impl From<Prop> for BaseItem {
    fn from(prop: Prop) -> Self {
        BaseItem::Prop(prop)
    }
}

impl From<Blueprint> for BaseItem {
    fn from(blueprint: Blueprint) -> Self {
        BaseItem::Blueprint(blueprint)
    }
}