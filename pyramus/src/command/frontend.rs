use strum_macros::Display;

#[derive(Display)]
pub enum FrontendCommand {
    /// Do not use directly.
    /// Should rerender the display
    /// This is never called directly, but used when we trigger a re-render of the stage
    /// (See the `should_rerender` method for more information)
    Rerender,

    /// The stage has been updated (e.g. items have been added, removed, or transformed)
    UpdateStage,
}

impl FrontendCommand {
    /// Whether the corresponding command should trigger a re-render of the stage
    pub fn should_rerender(&self) -> bool {
        match self {
            // Re-render should not trigger a re-render
            FrontendCommand::Rerender => false,

            FrontendCommand::UpdateStage => true,
        }
    }
}
