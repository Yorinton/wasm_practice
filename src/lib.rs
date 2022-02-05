extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: u8);
}

#[wasm_bindgen]
pub fn greet(name: &str, age: u8) {
    let mut v: Vec<i64> = vec![];
    for n in 1..50000000 {
        v.push(n);
    }
    log_many(name,age);
}
