use crate::{item::StagedItemId, models::Stage};

pub enum BackendCommand {
    DeleteItem(StagedItemId),
}

impl BackendCommand {
    pub fn process(&self, stage: &mut Stage) -> crate::Result<()> {
        match self {
            BackendCommand::DeleteItem(item_id) => stage.remove_item(*item_id),
        }
    }
}

pub enum FrontendCommand {
    // Trigger re-rendering of the canvas
    // TODO: unused
    Render,
}
