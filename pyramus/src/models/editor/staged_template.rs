use std::collections::HashMap;

use crate::models::templates::{
    ids::ItemId, prop::Prop, prop_builder::PropItemBuilder, prop_item::PropItem,
    transform::RelativeTransform,
};

use super::{item::StageItem, stage::StageItemBuilder};

// TODO: We might want to change this trait so that instead of having to manually
// handle each child/part/edit, we have a struct that represents all these shared fields.
pub trait StagedTemplate {
    // Item is a StagedItem, and Item::Template must refer back to this trait
    type Item: StageItem<Template = Self>;
    type ItemBuilder: StageItemBuilder;

    fn add_child(&mut self, item_builder: Self::ItemBuilder) -> crate::Result<ItemId>;

    fn edit_item(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut Self::Item) -> crate::Result<()>,
    ) -> crate::Result<()>;

    fn edit_item_transform(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut RelativeTransform) -> crate::Result<()>,
    ) -> crate::Result<()>;

    fn get_item(&self, id: ItemId) -> Option<&Self::Item>;

    fn get_items(&self) -> &HashMap<ItemId, Self::Item>;

    fn get_root(&self) -> ItemId;

    fn get_size(&self) -> (u32, u32);

    fn remove_item(&mut self, id: ItemId) -> crate::Result<()>;
}

impl StagedTemplate for Prop {
    type Item = PropItem;
    type ItemBuilder = PropItemBuilder;

    // TODO: This pattern could be improved (taking in an Rc<RwLock> parent, rather than a reference to self)
    fn add_child(&mut self, item_builder: Self::ItemBuilder) -> crate::Result<ItemId> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        let parent = item_builder.parent.unwrap_or(self.root);

        // Check parent exists and add child
        let parent = self
            .items
            .get_mut(&parent)
            .ok_or_else(|| crate::PyramusError::OtherError("Parent not found".to_string()))?;

        // Build and insert item
        let item = item_builder.build()?;
        let id = item.id;

        // Insert parent and child
        parent.children.push(id);
        self.items.insert(id, item);

        Ok(id)
    }

    fn edit_item(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut Self::Item) -> crate::Result<()>,
    ) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        if let Some(item) = self.items.get_mut(&id) {
            f(item)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    fn get_item(&self, id: ItemId) -> Option<&Self::Item> {
        self.items.get(&id)
    }

    fn get_items(&self) -> &HashMap<ItemId, Self::Item> {
        &self.items
    }

    fn get_root(&self) -> ItemId {
        self.root
    }

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn edit_item_transform(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut RelativeTransform) -> crate::Result<()>,
    ) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot edit the root item
        if id == self.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot edit the root item".to_string(),
            ));
        }

        if let Some(item) = self.items.get_mut(&id) {
            f(&mut item.transform)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    fn remove_item(&mut self, id: ItemId) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot remove the root item
        if id == self.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot remove the root item".to_string(),
            ));
        }

        let root = self.root;
        let parent = self.items.get(&id).and_then(|item| item.parent);
        let children = self
            .items
            .get(&id)
            .map(|item| item.children.clone())
            .unwrap_or_default();

        if let Some(parent) = parent.and_then(|parent| self.items.get_mut(&parent)) {
            parent.children.retain(|child| *child != id);
        }

        // Children should be kept and re-parented to the root
        // TODO: make this optional
        // If we reparent to the root, we should also calculate the new relative transform to keep the same position
        for child in children {
            let Some(child) = self.items.get_mut(&child) else {
                continue;
            };
            child.parent = Some(root);
        }
        self.items.remove(&id);

        Ok(())
    }
}
