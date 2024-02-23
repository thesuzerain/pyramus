use super::item::{
    Item, ItemImage, ItemText, RelativeTransform, StagedItem, StagedItemId,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Stage {
    pub size: (u32, u32),
    // TODO: It might be better to use a SlotMap/Arena here, rather than just tree structure
    // TODO: Also/alternatively, we should be using async RwLocks here, to allow for async rendering, probably
    pub root: StagedItemId,
    pub items: HashMap<StagedItemId, StagedItem>,
    pub selection: Vec<StagedItemId>,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            size: (800, 600),
            root: StagedItemId::new(),
            items: HashMap::new(),
            selection: Vec::new(),
        }
    }
}

// TODO: Move these functions to separate modules/files
impl Stage {
    // TODO: Background color should be a color type (consistency with other parts of the codebase)
    // TODO: Background color should be optional, or a pattern (like how Photoshop does transparency)
    pub fn build(width: u32, height: u32) -> crate::Result<Stage> {
        let root_id = StagedItemId::new();
        let root = StagedItem {
            id: root_id,
            name: "root".to_string(),
            item: Item::Image(ItemImage::from_rect(width, height, "red", None, 1.0)?),
            children: Vec::new(),
            parent: None,
            transform: RelativeTransform::default(),
        };
        let mut items = HashMap::new();
        items.insert(root_id, root);

        Ok(Stage {
            size: (width, height),
            root: root_id,
            items,
            selection: Vec::new(),
        })
    }

    // TODO: This pattern could be improved (taking in an Rc<RwLock> parent, rather than a reference to self)
    pub fn add_child(
        &mut self,
        name: String,
        parent: Option<StagedItemId>,
        item: Item,
        transform: Option<RelativeTransform>,
    ) -> crate::Result<StagedItemId> {
        let id = StagedItemId::new();
        let parent = parent.unwrap_or(self.root);

        let item = StagedItem {
            id,
            name,
            item,
            children: Vec::new(),
            parent: Some(parent),
            transform: transform.unwrap_or_default(),
        };

        self.items.insert(id, item);

        let parent = self
            .items
            .get_mut(&parent)
            .ok_or_else(|| crate::PyramusError::OtherError("Parent not found".to_string()))?;
        parent.children.push(id);
        Ok(id)
    }

    pub fn edit_item(
        &mut self,
        id: StagedItemId,
        f: impl FnOnce(&mut StagedItem) -> crate::Result<()>,
    ) -> crate::Result<()> {
        if let Some(item) = self.items.get_mut(&id) {
            f(item)
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn edit_item_transform(
        &mut self,
        id: StagedItemId,
        transform: RelativeTransform,
    ) -> crate::Result<()> {
        // Cannot edit the root item
        if id == self.root {
            return Err(crate::PyramusError::OtherError(
                "Cannot edit the root item".to_string(),
            ));
        }

        if let Some(item) = self.items.get_mut(&id) {
            item.transform = transform;
            Ok(())
        } else {
            Err(crate::PyramusError::OtherError(
                "Item not found".to_string(),
            ))
        }
    }

    pub fn remove_item(&mut self, id: StagedItemId) -> crate::Result<()> {
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

    pub fn set_selection(&mut self, selection: Vec<StagedItemId>) {
        self.selection = selection;
    }

    pub fn get_selections(&self) -> Vec<&StagedItem> {
        self.selection
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect()
    }

    pub fn get_front_item_at(&self, x: f32, y: f32, include_root : bool) -> Option<StagedItemId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Caching will help this
        let render_ordered = self.get_render_order();
        crate::log!("Render ordered: {:?}", render_ordered);
        for item_id in render_ordered.into_iter().rev() {
            if !include_root && item_id == self.root {
                continue;
            }
            
            let item = self.items.get(&item_id).unwrap();
            if item.contains_point(x, y, self) {
                return Some(item_id);
            }
        }
        None
    }

    pub fn get_render_order(&self) -> Vec<StagedItemId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Maybe we should use a BTreeMap here, to keep the order sorted, or a VecDeque to keep the order, or PartialEq implemntation, or something
        // TODO: Caching will help this
        let mut render_order = Vec::new();
        
        fn get_render_order_recursive(
            stage: &Stage,
            item_id: StagedItemId) -> Vec<StagedItemId> {
            let item = stage.items.get(&item_id).unwrap();
            let mut render_order = vec![item_id];
            for child in &item.children {
                render_order.extend(get_render_order_recursive(stage, *child));
            }
            render_order
        }
        render_order.extend(get_render_order_recursive(self, self.root));

        render_order
    }
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage() -> crate::Result<Stage> {
    let mut stage = Stage::build(800, 600)?;

    // Add a simple translucent rectangle as the background
    stage.add_child(
        "Rectangle".to_string(),
        None,
        ItemImage::from_rect(300, 200, "blue", None, 0.5)?.into(),
        None,
    )?;

    // TODO: Easy way to center items within their parent/the stage
    // TODO: Render order (z-index)

    // Add example text and image
    let image = ItemImage::from_bytes(include_bytes!("../../../testimg.jpg").to_vec(), "jpg")?.map(|im| stage.add_child(
        "Image".to_string(),
        None,
        Item::Image(im),
        Some(RelativeTransform {
            position: (50.0, 50.0),
            scale: (0.5, 0.5),
            rotation: 45.0,
        }),
    )).transpose()?;

    // Add example text and image
    if let Some(image) = image {
        stage.add_child(
            "Text".to_string(),
            Some(image),
            Item::Text(ItemText::build("Hello, world!".to_string())),
            Some(RelativeTransform {
                position: (100.0, 50.0),
                scale: (1.0, 5.0),
                rotation: -90.0, // Perpendicular to the image, not the stage
            }),
        )?;
    }

    Ok(stage)
}
