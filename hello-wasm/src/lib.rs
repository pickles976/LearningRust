extern crate nalgebra as na;
use na::{Vector3, Matrix4};
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub fn create_solver(target_array: Array, origin_array: Array, angles_array: Array, axes_array: Array, radii_array: Array) {
    alert("Created IK solver!");
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn print_array(array: Array) {
    alert(&format!("{:?}!", array));
    alert(&format!("{:?}!", array.length()));
    alert(&format!("{:?}!", array.get(0).as_f64().unwrap()));
}

#[wasm_bindgen]
pub fn solve_gd(target_array: Array, origin_array: Array, angles_array: Array, axes_array: Array, radii_array: Array) -> js_sys::Float32Array {

    let target_vec: Vec<f32> = js_array_to_vec_f32(target_array);
    let origin_vec: Vec<f32> = js_array_to_vec_f32(origin_array);
    let angles_vec: Vec<f32> = js_array_to_vec_f32(angles_array);
    let radii_vec: Vec<f32> = js_array_to_vec_f32(radii_array);

    let axes_vec = js_array_to_vec_str(axes_array).iter().map(|ax| 
        {
            match (ax.as_str()) {
                "x" => 0.0,
                "y" => 0.0,
                "z" => 0.0,
                _ => 0.0,
            }
        });

    return js_sys::Float32Array::from(&target_vec[..])
}

#[wasm_bindgen]
pub fn print_matrix(target_array: Array) {

    let target_vec: Vec<f32> = js_array_to_vec_f32(target_array);
    let matrix: Matrix4<f32> = Matrix4::from_iterator(target_vec.into_iter());

    alert(&format!("{:?}", matrix));

}

fn js_array_to_vec_str(array: Array) -> Vec<String> {
    let mut new_vec: Vec<String> = vec![];
    for i in 0..array.length() {
        new_vec.push(array.get(i).as_string().unwrap() as String);
    }
    new_vec
}

fn js_array_to_vec_f32(array: Array) -> Vec<f32> {
    let mut new_vec: Vec<f32> = vec![];
    for i in 0..array.length() {
        new_vec.push(array.get(i).as_f64().unwrap() as f32);
    }
    new_vec
}