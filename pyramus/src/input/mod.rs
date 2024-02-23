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
                let item_id = stage.get_front_item_at(x, y, false);
                crate::log!("Found item: {:?}", item_id);
                let new_selection = if let Some(item_id) = item_id {
                    // If the current selection is exactly the same as the clicked item, then we should clear the selection
                    if stage.get_selections().len() == 1 && stage.get_selections()[0].id == item_id {
                        vec![]
                    } else {
                        vec![item_id]
                    }
                } else {
                    vec![]
                };
                BackendCommand::SetSelection(new_selection).process(stage)
            }
        }
    }
}