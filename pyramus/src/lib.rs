use std::sync::PoisonError;

use thiserror::Error;
use wasm_bindgen::JsValue;

pub mod command;
pub mod input;
pub mod logging;
pub mod models;
pub mod render;

pub type Result<T> = std::result::Result<T, PyramusError>;

#[derive(Error, Debug)]
pub enum PyramusError {
    #[error("No runtime found!")]
    NoRuntimeFound,

    #[error("USVG error: {0}")]
    USVGError(#[from] resvg::usvg::Error),

    #[error("Svg types error: {0}")]
    SvgTypesError(#[from] svgtypes::Error),

    #[error("Std error: {0}")]
    StdError(#[from] std::io::Error),

    #[error("Thread poison error")]
    PoisonError,

    #[error("JsValue error: {0}")]
    JsValue(String),

    #[error("Invalid size: {0}, {1}")]
    InvalidSize(f32, f32),

    #[error("Image parsing error: {0}")]
    ImageParsingError(#[from] image::ImageError),

    #[error("Other error: {0}")]
    OtherError(String),
}

impl<T> From<PoisonError<T>> for PyramusError {
    fn from(_: PoisonError<T>) -> Self {
        PyramusError::PoisonError
    }
}

impl From<JsValue> for PyramusError {
    fn from(value: JsValue) -> Self {
        PyramusError::JsValue(value.as_string().unwrap_or_default())
    }
}
