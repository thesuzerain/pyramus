use pyramus::{command::FrontendCommand, PyramusError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{JsError, JsValue};

type CallbacksMap = HashMap<String, js_sys::Function>;

thread_local! {
    // TODO: Should this be Rc, OnceCell, thread_local!, etc?
    // TODO: Does this even need to be global?
    pub(crate) static RUNTIME: Rc<RefCell<Option<EditorRuntime>>> = Rc::new(RefCell::new(None));
    pub(crate) static CALLBACKS: Rc<RefCell<CallbacksMap>> = Rc::new(RefCell::new(HashMap::new()));
}

// Resolve a BackendCommand, and dispatch any resulting FrontendCommands
pub fn command(
    commands: impl IntoIterator<Item = pyramus::command::BackendCommand>,
) -> Result<(), JsError> {
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

pub struct EditorRuntime {
    pub stage: pyramus::models::stage::Stage,
}

impl Default for EditorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorRuntime {
    pub fn new() -> EditorRuntime {
        EditorRuntime {
            // TODO: Load from file, etc
            // TODO: When no longer a prototype, this should not need to be unwrapped
            stage: pyramus::models::stage::example_stage()
                .inspect_err(|e| pyramus::log!("Err: {e}"))
                .unwrap_or_default(),
        }
    }

    pub fn command(
        &mut self,
        commands: impl IntoIterator<Item = pyramus::command::BackendCommand>,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        let mut frontend_commands = Vec::new();
        for command in commands {
            frontend_commands.append(&mut command.process(&mut self.stage)?);
        }

        Ok(frontend_commands)
    }

    pub fn input(
        &mut self,
        event: pyramus::input::InputEvent,
    ) -> Result<Vec<FrontendCommand>, JsError> {
        Ok(event.process(&mut self.stage)?)
    }

    pub fn render_string(&self) -> Result<String, JsError> {
        Ok(pyramus::render::render_string(&self.stage)?)
    }
}
