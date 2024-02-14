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

pub fn dispatch_frontend_command(
    js_callbacks: &CallbacksMap,
    command: FrontendCommand,
) -> Result<(), PyramusError> {
    let s: String = command.to_string();
    pyramus::log!(
        "Dispatching frontend command: {}, keys: {:?}",
        s,
        js_callbacks.keys()
    );
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
            stage: pyramus::models::stage::example_stage().unwrap(),
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

    pub fn render(&self, canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsError> {
        Ok(pyramus::render::render(&self.stage, canvas)?)
    }

    pub fn render_string(&self) -> Result<String, JsError> {
        Ok(pyramus::render::render_string(&self.stage)?)
    }
}
