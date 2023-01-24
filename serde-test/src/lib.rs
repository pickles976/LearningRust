use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
extern crate nalgebra as na;
use na::{Vector3, Matrix4};

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

    const target: Matrix4<f32> = Matrix4::new(  
        1.0,0.0,0.0,5.0,
        0.0,1.0,0.0,5.0,
        0.0,0.0,1.0,7.0,
        0.0,0.0,0.0,1.0  
    );

    // let point: Vec<Vector3<f32>> = serde_json::from_str(val).unwrap();
    let mat: String = serde_json::to_string(&target).unwrap();

    alert(&format!("Hello, {:?}!", mat));
}