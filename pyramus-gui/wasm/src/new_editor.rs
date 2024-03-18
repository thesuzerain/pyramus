use pyramus::{
    command::{BackendCommand, FrontendCommand},
    models::editor::stage::{example_stage, Stage},
    PyramusError,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsError;

use crate::{
    editor::dispatch_frontend_command,
    models::{FrontendItem, FrontendStage},
};

type CallbacksMap = HashMap<String, js_sys::Function>;

thread_local! {
    // TODO: Should this be Rc, OnceCell, thread_local!, etc?
    // TODO: Does this even need to be global?
    // TODO: This should be generic- on heap?
    pub(crate) static RUNTIME: Rc<RefCell<Option<Runtime>>> = Rc::new(RefCell::new(None));
    pub(crate) static CALLBACKS: Rc<RefCell<CallbacksMap>> = Rc::new(RefCell::new(HashMap::new()));
}

pub struct Runtime {
    pub stage: Box<dyn RuntimeStage>, // TODO: Enum to put on stack?
}

pub trait RuntimeStage {
    fn command(
        &mut self,
        commands: Vec<BackendCommand>, // TODO: impl IntoIterator?
    ) -> Result<Vec<FrontendCommand>, PyramusError>;
    fn input(
        &mut self,
        event: pyramus::input::InputEvent,
    ) -> Result<Vec<FrontendCommand>, PyramusError>;
    fn render_string(&self) -> Result<String, PyramusError>;
    fn to_frontend_stage(&self) -> FrontendStage;
}

impl RuntimeStage for Stage {
    fn command(
        &mut self,
        commands: Vec<BackendCommand>,
    ) -> Result<Vec<FrontendCommand>, PyramusError> {
        let mut frontend_response = vec![];
        for command in commands {
            frontend_response.extend(self.process_command(command)?);
        }
        Ok(frontend_response)
    }

    fn input(
        &mut self,
        event: pyramus::input::InputEvent,
    ) -> Result<Vec<FrontendCommand>, PyramusError> {
        self.process_event(event)
    }

    fn render_string(&self) -> Result<String, PyramusError> {
        pyramus::render::render_string(self)
    }

    fn to_frontend_stage(&self) -> FrontendStage {
        FrontendStage {
            items: self
                .base
                .get_items()
                .iter()
                .map(|(id, item)| (id.0, FrontendItem::from(item, self)))
                .collect::<HashMap<_, _>>(),
            selected: self.selection.iter().map(|id| id.0).collect(),
        }
    }
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            stage: Box::new(
                example_stage()
                    .inspect_err(|e| pyramus::log!("Err: {e}"))
                    .unwrap(),
            ), // TODO: Load from file, etc
               // TODO: When no longer a prototype, this should not need to be unwrapped
        }
    }

    pub fn command(
        &mut self,
        commands: Vec<BackendCommand>,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        Ok(self
            .stage //tODO: awkward
            .command(commands)?)
    }

    pub fn input(
        &mut self,
        event: pyramus::input::InputEvent,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        Ok(self.stage.input(event)?)
    }

    pub fn render_string(&self) -> Result<String, JsError> {
        Ok(self.stage.render_string()?)
    }
}

// Resolve a BackendCommand, and dispatch any resulting FrontendCommands
// TODO: turn this back to impl IntoIterator<Item = BackendCommand>
pub fn command(commands: Vec<BackendCommand>) -> Result<(), JsError> {
    let frontend_response = RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let responses = runtime
            .as_mut()
            .map(|runtime| runtime.stage.command(commands))
            .ok_or_else(|| pyramus::PyramusError::NoRuntimeFound)??;
        Ok::<Vec<_>, JsError>(responses)
    })?;

    CALLBACKS.with(|callbacks| {
        let js_callbacks = callbacks.borrow();
        for command in frontend_response {
            dispatch_frontend_command(&js_callbacks, command)?;
        }
        Ok::<(), JsError>(())
    })?;

    Ok(())
}
