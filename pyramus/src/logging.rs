use wasm_bindgen::prelude::*;

// This is a simple macro named `say_hello`.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log(&format!($($arg)*));
    }
}

// TODO: Macros. Look into using tracing for logging on WASM
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // TODO: Do we want a version with multiple arguments?
    // // Multiple arguments too!
    // #[wasm_bindgen(js_namespace = console, js_name = log)]
    // pub fn log_many(a: &str, b: &str);
}
