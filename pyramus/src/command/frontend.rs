use strum_macros::Display;

#[derive(Display)]
pub enum FrontendCommand {
    // Not called directly, but used when we trigger a re-render of the stage
    Rerender,

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
