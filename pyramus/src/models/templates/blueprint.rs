use js_sys::Math::random;
use serde::{Deserialize, Serialize};

use crate::models::editor::{
    item::StageItem,
    staged_template::{BaseItem, BaseTemplate},
};

use super::{builder::ItemBuilder, ids::ItemId, prop::Prop};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    pub name: String,
    pub root: ItemId,
    pub size: (u32, u32), // TODO: Should this crop to the total bounds of the items?

    pub template: BaseTemplate,
}

impl Blueprint {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        (0.0, 0.0, self.size.0 as f32, self.size.1 as f32)
    }

    pub fn build_random(name: impl ToString, width: u32, height: u32) -> Blueprint {
        let mut items = HashMap::new();
        let root = Prop::new("root", width, height);
        let id = root.id;
        items.insert(id, StageItem::Prop(root));

        // building a "new object"" so we use a baseitem as if it were in a stage
        // Todo: is this the way to do?
        let mut blueprint = BaseItem::Blueprint(Blueprint {
            name: name.to_string(),
            root: id,
            size: (width, height),
            template: BaseTemplate {
                items,
                root: id,
                size: (width, height),
            },
        });

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

        if let BaseItem::Blueprint(blueprint) = blueprint {
            blueprint
        } else {
            panic!("Failed to extract blueprint") // TODO: Handle error, or refactor
        }
    }
}
