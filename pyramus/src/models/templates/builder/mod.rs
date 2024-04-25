use crate::models::editor::{item::StageItem, staging::StagingContext};

use self::{prop_builder::PropTypeBuilder, prop_item_builder::PropItemTypeBuilder};

use super::{ids::InternalId, prop_item::PropItem, transform::RelativeTransform};

pub mod prop_builder;
pub mod prop_item_builder;

/// Builder for creating a new item
#[derive(Debug)]
pub struct ItemBuilder {
    pub name: String,

    pub builder: BuilderType,

    pub parent: Option<InternalId>,
    pub transform: RelativeTransform,
}

#[derive(Debug)]
pub enum BuilderType {
    PropItem(PropItemTypeBuilder),
    Prop(PropTypeBuilder),
}

impl ItemBuilder {
    /// Finish and build the item
    pub fn build(self) -> crate::Result<StageItem> {
        match self.builder {
            BuilderType::PropItem(item) => {
                let item = item.build()?;
                // TODO: remove from here
                let prop_item = PropItem {
                    name: self.name,
                    id: InternalId::new(),
                    staging: StagingContext::new(),
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
