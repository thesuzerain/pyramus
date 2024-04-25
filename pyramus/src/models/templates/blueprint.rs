use js_sys::Math::random;
use serde::{Deserialize, Serialize};

use crate::models::editor::{
    base_item::{Base, BaseItem, BaseTemplate},
    item::StageItem,
};

use super::{builder::ItemBuilder, prop::Prop};
use std::collections::HashMap;

/// A blueprint
/// A structure that contains one or more props, easily modifable to fit a variety of contexts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blueprint {
    pub name: String,

    /// BaseTemplate so we can implement BaseItem
    pub template: BaseTemplate,
}

impl Blueprint {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        (
            0.0,
            0.0,
            self.template.size.0 as f32,
            self.template.size.1 as f32,
        )
    }

    pub fn build_random(name: impl ToString, width: u32, height: u32) -> Blueprint {
        let mut items = HashMap::new();
        let root = Prop::new("root", width, height);
        let id = root.id;
        items.insert(id, StageItem::Prop(root));

        // building a "new object"" so we use a baseitem as if it were in a stage
        // Todo: is this the way to do?
        let mut blueprint = Base::new(Blueprint {
            name: name.to_string(),
            template: BaseTemplate {
                items,
                root: id,
                size: (width, height),
            },
        }.into());

        // Add 3 random props
        for _ in 0..3 {
            // tODO: standardize f64 and f32 across codebase
            let w = (width as f64 * random()) as u32;
            let h = (height as f64 * random()) as u32;
            blueprint
                .add_child(ItemBuilder::build_random_prop(w, h))
                .unwrap(); // TODO: Handle error
        }

        // Extract
        // TODO ugly
        // maybe this function could return BaseItem instead of Blueprint

        if let BaseItem::Blueprint(blueprint) = blueprint.item {
            blueprint
        } else {
            panic!("Failed to extract blueprint") // TODO: Handle error, or refactor
        }
    }
}
