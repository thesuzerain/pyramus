use pyramus::{
    cache::Cache, command::{BackendCommand, FrontendCommand}, models::{editor::stage::{example_stage_prop, Stage}, templates::ids::PyramusId}, PyramusError
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

    // TODO: Should this be hosted on the js side? Or even further in rust?
    pub cache : Cache,
}

impl Runtime {
    pub async fn start() -> Runtime {
        let mut cache = Cache::new();
        cache.fetch().await;
        Runtime {
            stage: example_stage_prop()
                .inspect_err(|e| pyramus::log!("Err: {e}"))
                .unwrap(),
            cache
        }
    }

    pub fn new_base_prop(&mut self) -> Result<(), PyramusError> {
        // TODO: This should become a call to the server. (Or a different function here entirely)
        let id = PyramusId::debug_new();

        let stage = example_stage_prop()
            .inspect_err(|e| pyramus::log!("Err: {e}"))?;
        // TODO: See 'TODO' in set_base
        self.cache.insert_base(id, stage.base.clone());

        self.stage = stage; 
        Ok(())
    }

    pub fn set_base(&mut self, id: PyramusId) -> Result<(), PyramusError> {
        let prop = self.cache.get_base(id).ok_or_else(|| PyramusError::JsValue("No cached base found".to_string()))?;
        
        // TODO: Perhaps this shouldn't clone, and should be a reference to the value in the cache (which could act as an arena)
        self.stage = Stage::new(prop.clone());
        Ok(())
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
