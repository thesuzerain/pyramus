use super::FrontendCommand;
use crate::models::{
    editor::stage::{Stage, Stageable},
    templates::{ids::ItemId, prop_builder::PropItemBuilder, transform::RelativeTransform},
};

pub enum BackendCommand {
    CreateItem { new_item: PropItemBuilder },

    // Selection
    SetSelection(Vec<ItemId>),

    // TODO: Should this be EditTransform?
    TranslateGroup(Vec<ItemId>, (f32, f32)),

    // Item Editing
    EditTransform(ItemId, RelativeTransform),
    RenameItem(ItemId, String),
    DeleteItem(ItemId),
}

impl BackendCommand {
    pub fn process(self, stage: &mut Stage) -> crate::Result<Vec<FrontendCommand>> {
        let frontend_commands = match self {
            Self::CreateItem { new_item } => {
                let item_id = stage.base.add_child(new_item)?;
                stage.set_selection(vec![item_id]);
                vec![FrontendCommand::UpdateStage]
            }
            Self::SetSelection(selection) => {
                stage.set_selection(selection);
                vec![FrontendCommand::UpdateStage]
            }
            Self::DeleteItem(item_id) => {
                stage.base.remove_item(item_id)?;
                vec![FrontendCommand::UpdateStage]
            }
            Self::EditTransform(item_id, transform) => {
                stage.base.edit_item_transform(item_id, |t| {
                    *t = transform;
                    Ok(())
                })?;
                vec![FrontendCommand::UpdateStage]
            }
            Self::RenameItem(item_id, name) => {
                stage.base.edit_item(item_id, |item| {
                    item.name = name;
                    Ok(())
                })?;
                vec![FrontendCommand::UpdateStage]
            }
            Self::TranslateGroup(item_ids, (x, y)) => {
                for item_id in item_ids {
                    stage.base.edit_item_transform(item_id, |t| {
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
