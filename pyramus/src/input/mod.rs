use crate::{
    command::{BackendCommand, FrontendCommand},
    models::editor::stage::Stage,
};

#[derive(Debug)]
pub enum MouseState {
    Idle,
    MouseDown(f32, f32),
    DraggingMovement,
}

impl MouseState {
    const DRAG_THRESHOLD: f32 = 5.0;

    pub fn update_from_movement(&mut self, x: f32, y: f32) -> crate::Result<()> {
        match self {
            Self::Idle => Ok(()),
            Self::MouseDown(start_x, start_y) => {
                let dx = x - *start_x;
                let dy = y - *start_y;
                if dx * dx + dy * dy > Self::DRAG_THRESHOLD * Self::DRAG_THRESHOLD {
                    *self = Self::DraggingMovement;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Self::DraggingMovement => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum InputEvent {
    MouseDown { x: f32, y: f32 },
    MouseUp,
    MouseMove { delta_x: f32, delta_y: f32 },
    // TODO: Some kind of drag event- may need to be more complex, see Graphite's FSM
    // TODO: Keyboard events
}

impl Stage {
    pub fn process_event(&mut self, event: InputEvent) -> crate::Result<Vec<FrontendCommand>> {
        match event {
            InputEvent::MouseDown { x, y } => {
                self.mouse_state = MouseState::MouseDown(x, y);
                Ok(handle_selection(self, x, y)?)
            }
            InputEvent::MouseUp => {
                let frontend_commands =
                    if let MouseState::MouseDown(start_x, start_y) = self.mouse_state {
                        handle_click(self, start_x, start_y)?
                    } else {
                        vec![]
                    };
                self.mouse_state = MouseState::Idle;
                Ok(frontend_commands)
            }
            InputEvent::MouseMove { delta_x, delta_y } => {
                self.mouse_state.update_from_movement(delta_x, delta_y)?;
                let frontend_commands = if let MouseState::DraggingMovement = self.mouse_state {
                    handle_drag(self, delta_x, delta_y)?
                } else {
                    vec![]
                };
                Ok(frontend_commands)
            }
        }
    }
}

fn handle_selection(stage: &mut Stage, x: f32, y: f32) -> crate::Result<Vec<FrontendCommand>> {
    // Find item at x, y
    let item_id = stage.get_front_item_at(x, y, false);

    if let Some(item_id) = item_id {
        if stage.selection.iter().any(|s| *s == item_id) {
            // If we click an item, and it's already selected, do nothing (perhaps it will be dragged, etc)
            Ok(vec![])
        } else {
            // If we click an item, and it's not already selected, select it
            Ok(stage.process_command(BackendCommand::SetSelection(vec![item_id]))?)
        }
    } else if stage.get_selections_mut().is_empty() {
        // If we click nothing, and we have no selection, do nothing
        // TODO: This should do a box select
        Ok(vec![])
    } else {
        // If we click nothing, and we have a selection, clear the selection
        Ok(stage.process_command(BackendCommand::SetSelection(vec![]))?)
    }
}

fn handle_click(stage: &mut Stage, x: f32, y: f32) -> crate::Result<Vec<FrontendCommand>> {
    let item_id = stage.get_front_item_at(x, y, false);

    // If we fully get through a click, and it's on an item, select it
    // This 'overrides' 'handle_selection' behaviour as we know we aren't dragging
    if let Some(item_id) = item_id {
        Ok(stage.process_command(BackendCommand::SetSelection(vec![item_id]))?)
    } else {
        Ok(vec![])
    }
}

fn handle_drag(
    stage: &mut Stage,
    delta_x: f32,
    delta_y: f32,
) -> crate::Result<Vec<FrontendCommand>> {
    stage.process_command(BackendCommand::TranslateGroup(
        stage.selection.clone(),
        (delta_x, delta_y),
    ))
}
