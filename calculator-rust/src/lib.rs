use wasm_bindgen::prelude::*;
pub mod js_serde;
pub mod my_struct;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
// #[wasm_bindgen]
// /// Does something cool
// pub fn greet_many(name: &str, name2: &str) {
//     alert(&format!("Hello, {} and {}!", name, name2));
// }

// #[wasm_bindgen]
// pub fn foo(a: u32, b: u32, c: u32) -> u32 {
//     a + b + c
//     // alert(&format!("Hello, {}!", name));
// }

// #[wasm_bindgen]
// /// it adds
// pub fn add(a: u32, b: u32) -> u32 {
//     a + b
// }
