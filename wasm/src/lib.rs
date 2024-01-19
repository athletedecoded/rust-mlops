use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Generate a random number
pub fn random() -> u32 {
    (js_sys::Math::random() * 100.0) as u32
}
