use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log(&format!($($arg)*));
    }
}

// TODO: Look into using tracing for logging on WASM
// TODO: multiple levels of logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
