use crate::models::editor::{
    base_item::{Base, BaseItem, BaseTemplate},
    staging::StagingContext,
};

use super::{builder::ItemBuilder, ids::InternalId, transform::RelativeTransform};
use js_sys::Math::random;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: remove clone

/// A prop
/// A structure that contains one or more items, and can be reused in multiple blueprints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prop {
    pub id: InternalId,
    pub name: String,

    pub template: BaseTemplate,
    pub staging: StagingContext,
}

impl Prop {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        let (w, h) = self.template.size;
        (0.0, 0.0, w as f32, h as f32)
    }

    /// Generate a new, empty prop  with a white translucent background
    pub fn new(name: impl ToString, width: u32, height: u32) -> Prop {
        let mut items = HashMap::new();
        let root_builder = ItemBuilder::build_image_from_rect(width, height, "white", None, 0.1);
        let root = root_builder.build().unwrap(); // TODO: Handle error
        let id = root.get_id();
        items.insert(root.get_id(), root);

        let template = BaseTemplate {
            items,
            root: id,
            size: (width, height),
        };

        Prop {
            id: InternalId::new(),
            name: name.to_string(),
            template,
            staging: StagingContext::new(),
        }
    }

    // TODO: Remove, this is just to generate random props for testing
    /// Generate a new prop with a random structure
    pub fn build_random(name: impl ToString, width: u32, height: u32) -> Prop {
        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;

        // TODO: commentate
        let prop = Prop::new(name, width, height);
        let mut prop = Base::new(prop.into());
        // building a "new object"" so we use a baseitem as if it were in a stage

        // Add a randomly sized translucent rectangle as the background
        let rect_width = 100 + (random() * 200.0) as u32;
        let rect_height = 100 + (random() * 200.0) as u32;
        let rect_dx = center_x - (rect_width as i32 / 2);
        let rect_dy = center_y - (rect_height as i32 / 2);

        let rect = prop
            .add_child(
                ItemBuilder::build_image_from_rect(rect_width, rect_height, "blue", None, 0.5)
                    .name("Rectangle")
                    .transform(RelativeTransform {
                        position: (rect_dx as f32, rect_dy as f32),
                        ..Default::default()
                    }),
            )
            .unwrap(); // TODO: Handle error

        // Add image to the rectangle
        let image = prop
            .add_child(
                ItemBuilder::build_image_from_bytes(
                    include_bytes!("../../../../res/testimg.jpg").to_vec(),
                    "jpg",
                )
                .parent(rect)
                .name("Image")
                .transform(RelativeTransform::build_random(100)),
            )
            .unwrap(); // TODO: Handle error

        // Add example text
        prop.add_child(
            ItemBuilder::build_text_basic("Hello, world!")
                .name("Text")
                .parent(image)
                .transform(RelativeTransform::build_random(100)),
        )
        .unwrap(); // TODO: Handle error

        // Extract
        // TODO: This is... hacky. Maybe .add_child can be put on prop directly?
        if let BaseItem::Prop(prop) = prop.item {
            prop
        } else {
            panic!("Failed to extract prop") // TODO: Handle error, or refactor
        }
    }
}
