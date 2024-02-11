use crate::{item::StagedItemId, models::Stage};
use strum_macros::Display;

pub enum BackendCommand {
    DeleteItem(StagedItemId),
}

impl BackendCommand {
    pub fn process(&self, stage: &mut Stage) -> crate::Result<Vec<FrontendCommand>> {
        let frontend_commands = match self {
            BackendCommand::DeleteItem(item_id) => {
                stage.remove_item(*item_id)?;
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

#[derive(Display)]
pub enum FrontendCommand {
    // Not called directly, but used when we trigger a re-render of the stage
    Rerender,

    // TODO: unused
    UpdateStage,
}

impl FrontendCommand {
    pub fn should_rerender(&self) -> bool {
        match self {
            FrontendCommand::Rerender => false,

            FrontendCommand::UpdateStage => true,
        }
    }
}
