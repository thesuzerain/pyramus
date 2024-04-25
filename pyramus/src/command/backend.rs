use super::FrontendCommand;

use crate::models::{
    editor::stage::Stage,
    templates::{builder::ItemBuilder, ids::InternalId, transform::RelativeTransform},
};

/// A command that can be sent from the frontend to the backend, to
/// trigger some kind of change in the editor state.
pub enum BackendCommand {
    // TODO: Is there a way we can fuse these without giving BackendCommand a generic?
    // Adding the generic causes problems with the WASM layer holding a dyn Stage
    /// Create an item in the stage
    CreateItem { new_item: ItemBuilder },

    /// Change the list of selected items
    SetSelection(Vec<InternalId>),

    // TODO: Should this be EditTransform?
    /// Translate a group of items in a direction (x,y)
    TranslateGroup(Vec<InternalId>, (f32, f32)),

    /// Change the transform of an item to provided RelativeTransform
    EditTransform(InternalId, RelativeTransform),

    /// Rename an item
    RenameItem(InternalId, String),

    /// Delete an item
    DeleteItem(InternalId),
}

impl Stage {
    /// Process a command from the frontend, and return a list of FrontendCommands to be executed in response.
    pub fn process_command(
        &mut self,
        command: BackendCommand,
    ) -> crate::Result<Vec<FrontendCommand>> {
        let frontend_commands = match command {
            // TODO: Combine these? Only allow one?
            BackendCommand::CreateItem { new_item } => {
                let item_id = self.base.add_child(new_item)?;
                self.set_selection(vec![item_id]);
                vec![FrontendCommand::UpdateStage]
            }
            BackendCommand::SetSelection(selection) => {
                self.set_selection(selection);
                vec![FrontendCommand::UpdateStage]
            }
            BackendCommand::DeleteItem(item_id) => {
                self.base.remove_item(item_id)?;
                vec![FrontendCommand::UpdateStage]
            }
            BackendCommand::EditTransform(item_id, transform) => {
                self.base.edit_item_transform(item_id, |t| {
                    *t = transform;
                    Ok(())
                })?;
                vec![FrontendCommand::UpdateStage]
            }
            BackendCommand::RenameItem(item_id, name) => {
                self.base.edit_item(item_id, |item| {
                    item.rename(name);
                    Ok(())
                })?;
                vec![FrontendCommand::UpdateStage]
            }
            BackendCommand::TranslateGroup(item_ids, (x, y)) => {
                for item_id in item_ids {
                    // TODO: This pattern of going from stage -> base -> item -> function call is quite common. Might be a smell
                    self.base.edit_item_transform(item_id, |t| {
                        t.position.0 += x;
                        t.position.1 += y;
                        Ok(())
                    })?;
                }
                vec![FrontendCommand::UpdateStage]
            }
        };

        // If any require re-rendering, we should append FrontendCommand::Rerender to the end
        if frontend_commands.iter().any(|c| c.should_rerender()) {
            Ok(frontend_commands
                .into_iter()
                .chain(std::iter::once(FrontendCommand::Rerender))
                .collect())
        } else {
            Ok(frontend_commands)
        }
    }
}
