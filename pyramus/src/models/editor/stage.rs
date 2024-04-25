use super::{base_item::Base, item::StageItem, staging::Staging};
use crate::{
    input::MouseState,
    models::templates::{blueprint::Blueprint, ids::InternalId, prop::Prop},
};

/// The stage is the main area where items are placed and manipulated.
/// It is the main area of interaction for the user.
/// It is a container for any template:
/// - A prop (a collection of prop items)
/// - A blueprint (a collection of props)
#[derive(Debug)]
pub struct Stage {
    pub base: Base, // TODO: Should be able to be a blueprint or a prop. Stage<T> where T is 'stageable'
    pub selection: Vec<InternalId>,

    // TODO: Should be exported to some other 'state'-type mechansms
    pub mouse_state: MouseState,
}

// TODO: Move these functions to separate modules/files
impl Stage {
    // TODO: Background color should be a color type (consistency with other parts of the codebase)
    // TODO: Background color should be optional, or a pattern (like how Photoshop does transparency)

    // TODO: This should become trait again, with one build()
    /// Create a new stage from a base
    pub fn new(base: Base) -> Stage {
        Stage {
            base,
            selection: Vec::new(),
            mouse_state: MouseState::Idle,
        }
    }

    /// Sets selected items in the stage
    pub fn set_selection(&mut self, selection: Vec<InternalId>) {
        self.selection = selection;
    }

    /// Get the selected items in the stage, mutably
    pub fn get_selections_mut(&mut self) -> Vec<&StageItem> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        self.selection
            .iter()
            .filter_map(|id| self.base.get_item(*id))
            .collect()
    }

    /// Get the selected items in the stage
    pub fn get_selections(&self) -> Vec<&StageItem> {
        // TODO: Revisit this function after blueprint refactor- move to prop?
        self.selection
            .iter()
            .filter_map(|id| self.base.get_item(*id))
            .collect()
    }

    /// Get the front-most item at the given x,y (in screen coordinates)
    /// None if no item is found
    pub fn get_front_item_at(&self, x: f32, y: f32, include_root: bool) -> Option<InternalId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Caching will help this
        let render_ordered = self.get_render_order();
        for item_id in render_ordered.into_iter().rev() {
            if !include_root && item_id == self.base.get_root() {
                continue;
            }

            let item: &StageItem = self.base.get_item(item_id).unwrap(); // TODO: unwrap
            if item.contains_point(x, y, &self.base) {
                return Some(item_id);
            }
        }
        None
    }

    /// Get the render order of the items in the stage
    pub fn get_render_order(&self) -> Vec<InternalId> {
        // TODO: We need to add Z-index (render order) support, which will affect how this selects items
        // Currently, this just uses the children order (last child is on top), which should be used as a tiebreaker
        // TODO: Maybe we should use a BTreeMap here, to keep the order sorted, or a VecDeque to keep the order, or PartialEq implemntation, or something
        // TODO: Caching will help this
        let mut render_order = Vec::new();

        render_order.extend(Self::get_render_order_recursive(self, self.base.get_root()));

        render_order
    }

    fn get_render_order_recursive(stage: &Stage, item_id: InternalId) -> Vec<InternalId> {
        let item = stage.base.get_item(item_id).unwrap(); // TODO: Handle this unwrap properly
        let mut render_order = vec![item_id];
        for child in item.get_children() {
            render_order.extend(Self::get_render_order_recursive(stage, *child));
        }
        render_order
    }
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage_prop() -> crate::Result<Stage> {
    let prop = Prop::build_random("Test", 800, 600);
    let base = Base::new(prop.into());
    let stage = Stage::new(base);
    Ok(stage)
}

// TODO: Remove, this is just for testing of WASM rendering before other features are implemented
pub fn example_stage_blueprint() -> crate::Result<Stage> {
    let blueprint = Blueprint::build_random("Test", 800, 600);
    let base = Base::new(blueprint.into());
    let stage = Stage::new(base);
    Ok(stage)
}
