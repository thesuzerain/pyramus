use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsError;

thread_local! {
    // TODO: Should this be Arc, OnceCell, thread_local!, etc?
    // TODO: Does this even need to be global?
    pub(crate) static RUNTIME: Rc<RefCell<Option<EditorRuntime>>> = Rc::new(RefCell::new(None));
}

pub fn command(
    commands: impl IntoIterator<Item = pyramus::command::BackendCommand>,
) -> Result<(), JsError> {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        runtime
            .as_mut()
            .map(|runtime| runtime.command(commands))
            .ok_or_else(|| pyramus::PyramusError::NoRuntimeFound)?
    })
}

pub struct EditorRuntime {
    pub stage: pyramus::models::Stage,
}

impl EditorRuntime {
    pub fn new() -> EditorRuntime {
        EditorRuntime {
            // TODO: Load from file, etc
            // TODO: When no longer a prototype, this should not need to be unwrapped
            stage: pyramus::models::example_stage().unwrap(),
        }
    }

    pub fn command(
        &mut self,
        commands: impl IntoIterator<Item = pyramus::command::BackendCommand>,
    ) -> Result<(), JsError> {
        for command in commands {
            command.process(&mut self.stage)?;
        }

        Ok(())
    }

    pub fn render(&self, canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsError> {
        Ok(pyramus::render::render(&self.stage, canvas)?)
    }

    pub fn render_string(&self) -> Result<String, JsError> {
        Ok(pyramus::render::render_string(&self.stage)?)
    }
}
