use pyramus::{
    command::{BackendCommand, FrontendCommand},
    models::editor::stage::{example_stage_blueprint, example_stage_prop, Stage},
    PyramusError,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{JsError, JsValue};

type CallbacksMap = HashMap<String, js_sys::Function>;

thread_local! {
    // TODO: Should this be Rc, OnceCell, thread_local!, etc?
    // TODO: Does this even need to be global?
    // TODO: This should be generic- on heap?
    pub(crate) static RUNTIME: Rc<RefCell<Option<Runtime>>> = Rc::new(RefCell::new(None));
    pub(crate) static CALLBACKS: Rc<RefCell<CallbacksMap>> = Rc::new(RefCell::new(HashMap::new()));
}

pub struct Runtime {
    pub stage: Stage, // TODO: Enum to put on stack?
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            stage: example_stage_prop()
                .inspect_err(|e| pyramus::log!("Err: {e}"))
                .unwrap(),
        }
    }

    pub fn set_prop(&mut self) {
        self.stage = example_stage_prop()
            .inspect_err(|e| pyramus::log!("Err: {e}"))
            .unwrap();
        pyramus::log!("Set prop");
    }

    pub fn set_blueprint(&mut self) {
        self.stage = example_stage_blueprint()
            .inspect_err(|e| pyramus::log!("Err: {e}"))
            .unwrap();
        pyramus::log!("Set blueprint");
    }

    pub fn command(
        &mut self,
        commands: Vec<BackendCommand>,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        let mut frontend_response = vec![];
        for command in commands {
            frontend_response.extend(self.stage.process_command(command)?);
        }
        Ok(frontend_response)
    }

    pub fn input(
        &mut self,
        event: pyramus::input::InputEvent,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        Ok(self.stage.process_event(event)?)
    }

    pub fn render_string(&self) -> Result<String, JsError> {
        Ok(pyramus::render::render_string(&self.stage)?)
    }
}

// Resolve a BackendCommand, and dispatch any resulting FrontendCommands
// TODO: turn this back to impl IntoIterator<Item = BackendCommand>
pub fn command(commands: Vec<BackendCommand>) -> Result<(), JsError> {
    let frontend_response = RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let responses = runtime
            .as_mut()
            .map(|runtime| runtime.command(commands))
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

pub fn dispatch_frontend_command(
    js_callbacks: &CallbacksMap,
    command: FrontendCommand,
) -> Result<(), PyramusError> {
    let s: String = command.to_string();
    match js_callbacks.get(&s) {
        Some(callback) => {
            callback
                .call1(&JsValue::NULL, &JsValue::from_str(&s))
                .map(pyramus::PyramusError::from)?;
        }
        None => {
            // TODO: Should this be an error?
            pyramus::log!("No callback found for command: {}", s);
        }
    }
    Ok(())
}

// Process an InputEvent, and dispatch any resulting FrontendCommands
// InputEvents are translated into BackendCommands behind the scenes, and also result in FrontendCommands,
// but may have additional behaviour that is not related to commands
pub fn input(event: pyramus::input::InputEvent) -> Result<(), JsError> {
    let frontend_response = RUNTIME.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let responses = runtime
            .as_mut()
            .map(|runtime| runtime.input(event))
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
