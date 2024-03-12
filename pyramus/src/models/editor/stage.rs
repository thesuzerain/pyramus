use crate::{
    input::MouseState,
    models::blueprint::{
        ids::ItemId,
        prop::{Prop, PropItem, PropItemImage, PropItemText, PropItemType},
        transform::RelativeTransform,
    },
};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Stage {
    pub base: Prop, // TODO: Should be able to be a blueprint or a prop. Stage<T> where T is 'stageable'
    pub selection: Vec<ItemId>,

    // TODO: Should be exported to some other 'state'-type mechansms
    pub mouse_state: MouseState,

    pub stage_type: StageType, // TODO: Might be unecessary with 'base'
}

#[derive(Debug)]
pub enum StageType {
    PropEditor,
    BlueprintEditor,
    Display,
}

// TODO: Move these functions to separate modules/files
impl Stage {
    // TODO: Background color should be a color type (consistency with other parts of the codebase)
    // TODO: Background color should be optional, or a pattern (like how Photoshop does transparency)
    pub fn build(width: u32, height: u32, stage_type: StageType) -> crate::Result<Stage> {
        // TODO: Revisit this function after blueprint refactor
        let id = ItemId::new();
        let root = PropItem {
            id,
            name: "root".to_string(),
            item: PropItemType::Image(PropItemImage::from_rect(width, height, "red", None, 1.0)?),
            children: Vec::new(),
            parent: None,
            transform: RelativeTransform::default(),
        };

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
            stage_type,
        })
    }

    // TODO: This pattern could be improved (taking in an Rc<RwLock> parent, rather than a reference to self)
    pub fn add_child(
        &mut self,
        name: String,
        parent: Option<ItemId>,
        item: PropItemType, // TODO: This is unintuitive
        transform: Option<RelativeTransform>,
    ) -> crate::Result<ItemId> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        let id = ItemId::new();
        let parent = parent.unwrap_or(self.base.root);

        let item = PropItem {
            id,
            name,
            item,
            children: Vec::new(),
            parent: Some(parent),
            transform: transform.unwrap_or_default(),
        };

        self.base.items.insert(id, item);

        let parent = self
            .base
            .items
            .get_mut(&parent)
            .ok_or_else(|| crate::PyramusError::OtherError("Parent not found".to_string()))?;
        parent.children.push(id);
        Ok(id)
    }

    pub fn edit_item(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut PropItem) -> crate::Result<()>,
    ) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        if let Some(item) = self.base.items.get_mut(&id) {
            f(item)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn edit_item_transform(
        &mut self,
        id: ItemId,
        f: impl FnOnce(&mut RelativeTransform) -> crate::Result<()>,
    ) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot edit the root item
        if id == self.base.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot edit the root item".to_string(),
            ));
        }

        if let Some(item) = self.base.items.get_mut(&id) {
            f(&mut item.transform)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn remove_item(&mut self, id: ItemId) -> crate::Result<()> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        // Cannot remove the root item
        if id == self.base.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot remove the root item".to_string(),
            ));
        }

        let root = self.base.root;
        let parent = self.base.items.get(&id).and_then(|item| item.parent);
        let children = self
            .base
            .items
            .get(&id)
            .map(|item| item.children.clone())
            .unwrap_or_default();

        if let Some(parent) = parent.and_then(|parent| self.base.items.get_mut(&parent)) {
            parent.children.retain(|child| *child != id);
        }

        // Children should be kept and re-parented to the root
        // TODO: make this optional
        // If we reparent to the root, we should also calculate the new relative transform to keep the same position
        for child in children {
            let Some(child) = self.base.items.get_mut(&child) else {
                continue;
            };
            child.parent = Some(root);
        }
        self.base.items.remove(&id);

        Ok(())
    }

    pub fn set_selection(&mut self, selection: Vec<ItemId>) {
        self.selection = selection;
    }

    pub fn get_selections(&self) -> Vec<&PropItem> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        self.selection
            .iter()
            .filter_map(|id| self.base.items.get(id))
            .collect()
    }

    pub fn get_front_item_at(&self, x: f32, y: f32, include_root: bool) -> Option<ItemId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Caching will help this
        let render_ordered = self.get_render_order();
        crate::log!("Render ordered: {:?}", render_ordered);
        for item_id in render_ordered.into_iter().rev() {
            if !include_root && item_id == self.base.root {
                continue;
            }

            let item = self.base.items.get(&item_id).unwrap(); // TODO: unwrap
            if item.contains_point(x, y, self) {
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

        fn get_render_order_recursive(stage: &Stage, item_id: ItemId) -> Vec<ItemId> {
            let item = stage.base.items.get(&item_id).unwrap();
            let mut render_order = vec![item_id];
            for child in &item.children {
                render_order.extend(get_render_order_recursive(stage, *child));
            }
            render_order
        }
        render_order.extend(get_render_order_recursive(self, self.base.root));

        render_order
    }
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage() -> crate::Result<Stage> {
    let mut stage = Stage::build(800, 600, StageType::PropEditor)?;

    // Add a simple translucent rectangle as the background
    stage.add_child(
        "Rectangle".to_string(),
        None,
        PropItemImage::from_rect(300, 200, "blue", None, 0.5)?.into(),
        None,
    )?;

    // TODO: Easy way to center items within their parent/the stage
    // TODO: Render order (z-index)

    // Add example text and image
    let image =
        PropItemImage::from_bytes(include_bytes!("../../../../testimg.jpg").to_vec(), "jpg")?
            .map(|im| {
                stage.add_child(
                    "Image".to_string(),
                    None,
                    PropItemType::Image(im),
                    Some(RelativeTransform {
                        position: (50.0, 50.0),
                        scale: (0.5, 0.5),
                        rotation: 45.0,
                    }),
                )
            })
            .transpose()?;

    // Add example text and image
    if let Some(image) = image {
        stage.add_child(
            "Text".to_string(),
            Some(image),
            PropItemType::Text(PropItemText::build("Hello, world!".to_string())),
            Some(RelativeTransform {
                position: (100.0, 50.0),
                scale: (1.0, 5.0),
                rotation: -90.0, // Perpendicular to the image, not the stage
            }),
        )?;
    }

    Ok(stage)
}
