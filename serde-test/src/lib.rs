use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
extern crate nalgebra as na;
use na::Vector3;

#[cfg(feature = "nalgebra/serde-serialize")]

#[derive(Serialize, Deserialize, Debug)]
struct Position<T> {
    x: T,
    y: T,
    z: T
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(val: &str) {

    let point: Vec<Vector3<f32>> = serde_json::from_str(val).unwrap();

    alert(&format!("Hello, {:?}!", point));
}