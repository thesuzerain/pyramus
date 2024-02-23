use crate::{command::{BackendCommand, FrontendCommand}, models::stage::Stage};

pub enum InputEvent {
    Click { x: f32, y: f32 },
    // TODO: Some kind of drag event- may need to be more complex, see Graphite's FSM system
    // TODO: Keyboard events
}

impl InputEvent {
    pub fn process(self, stage: &mut Stage) -> crate::Result<Vec<FrontendCommand>> {
        match self {
            Self::Click { x, y } => {
                let item_id = stage.get_item_at(x, y);
                crate::log!("Found item: {:?}", item_id);
                if let Some(item_id) = item_id {
                    BackendCommand::SetSelection(vec![item_id]).process(stage)
                } else {
                    BackendCommand::SetSelection(vec![]).process(stage)
                }
            }
        }
    }
}