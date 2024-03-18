use pyramus::{command::FrontendCommand, PyramusError};
use std::collections::HashMap;
use wasm_bindgen::{JsError, JsValue};

use crate::new_editor::{CALLBACKS, RUNTIME};

type CallbacksMap = HashMap<String, js_sys::Function>;

thread_local! {
    // TODO: Should this be Rc, OnceCell, thread_local!, etc?
    // TODO: Does this even need to be global?
    // TODO: This should be generic- on heap?
    // pub(crate) static RUNTIME: Rc<RefCell<Option<EditorRuntime>>> = Rc::new(RefCell::new(None));
    // pub(crate) static CALLBACKS: Rc<RefCell<CallbacksMap>> = Rc::new(RefCell::new(HashMap::new()));
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
