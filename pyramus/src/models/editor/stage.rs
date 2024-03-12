use super::{item::StageItem, staged_template::StagedTemplate};
use crate::{
    input::MouseState,
    models::templates::{
        ids::ItemId, prop::Prop, prop_builder::PropItemBuilder, transform::RelativeTransform,
    },
};
use std::collections::HashMap;

/// The stage is the main area where items are placed and manipulated.
/// It is the main area of interaction for the user.
/// It is a container for any template:
/// - A prop (a collection of prop items)
/// - A blueprint (a collection of props)
#[derive(Debug)]
pub struct Stage<T: StagedTemplate> {
    pub base: T, // TODO: Should be able to be a blueprint or a prop. Stage<T> where T is 'stageable'
    pub selection: Vec<ItemId>,

    // TODO: Should be exported to some other 'state'-type mechansms
    pub mouse_state: MouseState,
}

pub trait StageItemBuilder {
    type Item: StageItem;

    fn build(self) -> crate::Result<Self::Item>;
}

impl Stage<Prop> {
    // TODO: Background color should be a color type (consistency with other parts of the codebase)
    // TODO: Background color should be optional, or a pattern (like how Photoshop does transparency)
    pub fn build(width: u32, height: u32) -> crate::Result<Stage<Prop>> {
        // TODO: Revisit this function after blueprint refactor
        let root = PropItemBuilder::build_image_from_rect(width, height, "red", None, 1.0)
            .name("root")
            .build()?;
        let id = root.id;

        let mut items = HashMap::new();
        items.insert(root.id, root);

        let base: Prop = Prop {
            name: "temp".to_string(),
            items,
            root: id,
            size: (width, height),
        };

        Ok(Stage {
            base,
            selection: Vec::new(),
            mouse_state: MouseState::Idle,
        })
    }
}

// TODO: Move these functions to separate modules/files
impl<T: StagedTemplate> Stage<T> {
    pub fn set_selection(&mut self, selection: Vec<ItemId>) {
        self.selection = selection;
    }

    pub fn get_selections(&self) -> Vec<&T::Item> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        self.selection
            .iter()
            .filter_map(|id| self.base.get_item(*id))
            .collect()
    }

    pub fn get_front_item_at(&self, x: f32, y: f32, include_root: bool) -> Option<ItemId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Caching will help this
        let render_ordered = self.get_render_order();
        crate::log!("Render ordered: {:?}", render_ordered);
        for item_id in render_ordered.into_iter().rev() {
            if !include_root && item_id == self.base.get_root() {
                continue;
            }

            let item: &T::Item = self.base.get_item(item_id).unwrap(); // TODO: unwrap
            if item.contains_point(x, y, self as &Stage<T>) {
                return Some(item_id);
            }
        }
        None
    }

    pub fn get_render_order(&self) -> Vec<ItemId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Maybe we should use a BTreeMap here, to keep the order sorted, or a VecDeque to keep the order, or PartialEq implemntation, or something
        // TODO: Caching will help this
        let mut render_order = Vec::new();

        render_order.extend(Self::get_render_order_recursive(self, self.base.get_root()));

        render_order
    }

    fn get_render_order_recursive(stage: &Stage<T>, item_id: ItemId) -> Vec<ItemId> {
        let item = stage.base.get_item(item_id).unwrap(); // TODO: Handle this unwrap properly
        let mut render_order = vec![item_id];
        for child in item.get_children() {
            render_order.extend(Self::get_render_order_recursive(stage, *child));
        }
        render_order
    }
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage() -> crate::Result<Stage<Prop>> {
    let mut stage = Stage::build(800, 600)?;

    // Add a simple translucent rectangle as the background
    let rect = stage.base.add_child(
        PropItemBuilder::build_image_from_rect(300, 200, "blue", None, 0.5).name("Rectangle"),
    )?;

    // TODO: Easy way to center items within their parent/the stage
    // TODO: Render order (z-index)

    // Add example text and image
    let image = stage.base.add_child(
        PropItemBuilder::build_image_from_bytes(
            include_bytes!("../../../../testimg.jpg").to_vec(),
            "jpg",
        )
        .parent(rect)
        .name("Image")
        .transform(RelativeTransform {
            position: (50.0, 50.0),
            scale: (0.5, 0.5),
            rotation: 45.0,
        }),
    )?;

    // Add example text and image
    stage.base.add_child(
        PropItemBuilder::build_text_basic("Hello, world!")
            .name("Text")
            .parent(image)
            .transform(RelativeTransform {
                position: (100.0, 50.0),
                scale: (1.0, 5.0),
                rotation: -90.0, // Perpendicular to the image, not the stage
            }),
    )?;

    Ok(stage)
}
