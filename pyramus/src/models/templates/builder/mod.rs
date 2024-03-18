use crate::models::editor::item::StageItem;

use self::{prop_builder::PropTypeBuilder, prop_item_builder::PropItemTypeBuilder};

use super::{ids::ItemId, prop_item::PropItem, transform::RelativeTransform};

pub mod prop_builder;
pub mod prop_item_builder;

#[derive(Debug)]
pub struct ItemBuilder {
    pub name: String,

    pub builder: BuilderType,

    pub parent: Option<ItemId>,
    pub transform: RelativeTransform,
}

#[derive(Debug)]
pub enum BuilderType {
    PropItem(PropItemTypeBuilder),
    Prop(PropTypeBuilder),
}

impl ItemBuilder {
    pub fn build(self) -> crate::Result<StageItem> {
        match self.builder {
            BuilderType::PropItem(item) => {
                let item = item.build()?;
                // TODO: remove from here
                let prop_item = PropItem {
                    name: self.name,
                    id: ItemId::new(),
                    parent: self.parent,
                    transform: self.transform,
                    children: vec![],
                    item,
                };
                Ok(StageItem::PropItem(prop_item))
            }
            BuilderType::Prop(prop) => {
                let prop = prop.build(self.name);
                Ok(StageItem::Prop(prop))
            }
        }
    }
}
