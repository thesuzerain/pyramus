use crate::models::editor::staged_template::{BaseItem, BaseTemplate};

use super::{builder::ItemBuilder, ids::ItemId, transform::RelativeTransform};
use js_sys::Math::random;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Prop {
    pub id: ItemId,
    pub name: String,

    pub template: BaseTemplate,

    // Stageable objects (TODO: Move to a separate struct?)
    // Within a blueprint, if applicable, or parent
    pub parent: Option<ItemId>,
    pub children: Vec<ItemId>,
    pub transform: RelativeTransform,
}

impl Prop {
    // x0, y0, x1, y1
    pub fn get_local_bounds(&self) -> (f32, f32, f32, f32) {
        let (w, h) = self.template.size;
        (0.0, 0.0, w as f32, h as f32)
    }

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
            id: ItemId::new(),
            name: name.to_string(),
            template,
            parent: None,
            children: vec![],
            transform: RelativeTransform::default(),
        }
    }

    // TODO: Remove, this is just to generate random props for testing
    pub fn build_random(name: impl ToString, width: u32, height: u32) -> Prop {
        let center_x = width / 2;
        let center_y = height / 2;

        // TODO: commentate
        let prop = Prop::new(name, width, height);
        let mut prop = BaseItem::Prop(prop);
        // building a "new object"" so we use a baseitem as if it were in a stage

        // Add a randomly sized translucent rectangle as the background
        let rect_width = 100 + (random() * 200.0) as u32;
        let rect_height = 100 + (random() * 200.0) as u32;

        let rect_dx = center_x - (rect_width / 2);
        let rect_dy = center_y - (rect_height / 2);

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
                    include_bytes!("../../../../testimg.jpg").to_vec(),
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
        let BaseItem::Prop(prop) = prop;
        prop
    }
}
