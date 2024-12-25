use crate::js_serde::JsValueSerdeExt;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
#[wasm_bindgen]
// #[wasm_bindgen(implements_trait = "")]
pub struct MyStruct {
	a: u32,
	b: String,
	c: Vec<MyStruct>,
}

#[wasm_bindgen]
impl MyStruct {
	// #[wasm_bindgen(constructor)]
	pub fn new(a: u32, b: String, c: Vec<MyStruct>) -> Self { Self { a, b, c } }

	pub fn arr_len(&self) -> usize { self.c.len() }

	pub fn a(&self) -> u32 { self.a }
	pub fn b(&self) -> String { self.b.clone() }
	pub fn c(&self) -> Vec<MyStruct> { self.c.clone() }
}

#[wasm_bindgen]
/// it checks the struct
pub fn check_my_struct(val: JsValue) -> bool {
	match val.into_serde::<MyStruct>() {
		Ok(_v) => true,
		Err(e) => false,
	}
}
#[wasm_bindgen]
/// it checks the struct
pub fn check_my_struct2(val: MyStruct) -> MyStruct {
	val
	// match val.into_serde::<MyStruct>() {
	// 	Ok(_v) => true,
	// 	Err(e) => false,
	// }
}


// fn from_js<T: Deserialize>(js: JsValue) -> T { js.into_serde().unwrap() }
