use super::FrontendCommand;
use crate::models::{
    item::{ItemBuilder, RelativeTransform, StagedItemId},
    stage::Stage,
};

pub enum BackendCommand {
    CreateItem {
        name: String,
        parent: StagedItemId,
        new_item: ItemBuilder,
    },

    // Selection
    SetSelection(Vec<StagedItemId>),

    // Item Editing
    EditTransform(StagedItemId, RelativeTransform),
    RenameItem(StagedItemId, String),
    DeleteItem(StagedItemId),
}

impl BackendCommand {
    pub fn process(self, stage: &mut Stage) -> crate::Result<Vec<FrontendCommand>> {
        let frontend_commands = match self {
            Self::CreateItem {
                name,
                parent,
                new_item,
            } => {
                let item = new_item.build()?;
                let item_id = stage.add_child(name, Some(parent), item, None)?;
                stage.set_selection(vec![item_id]);
                vec![FrontendCommand::UpdateStage]
            }
            Self::SetSelection(selection) => {
                stage.set_selection(selection);
                vec![FrontendCommand::UpdateStage]
            }
            Self::DeleteItem(item_id) => {
                stage.remove_item(item_id)?;
                vec![FrontendCommand::UpdateStage]
            }
            Self::EditTransform(item_id, transform) => {
                stage.edit_item_transform(item_id, transform)?;
                vec![FrontendCommand::UpdateStage]
            }
            Self::RenameItem(item_id, name) => {
                stage.edit_item(item_id, |item| {
                    item.name = name;
                    Ok(())
                })?;
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
