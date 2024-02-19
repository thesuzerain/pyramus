use wasm_bindgen::prelude::*;

/// Provides pointer to the start of the image buffer
#[wasm_bindgen(js_name = imageBufferPointer)]
pub fn image_buffer_pointer() -> *const u8 {
    unsafe { OUTPUT_BUFFER.as_ptr() }
}

// TODO: Look at this again- checkerboard was for testing, but is now unused
const CHECKERBOARD_SIZE: usize = 20;

const OUTPUT_BUFFER_SIZE: usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];
