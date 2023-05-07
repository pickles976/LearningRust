use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InverseKinematics {
    solver: i32,
    collision_handler: f64,
}

#[wasm_bindgen]
impl InverseKinematics {
    pub fn new(val: i32, col: f64) -> InverseKinematics {
        InverseKinematics { 
            solver: val, 
            collision_handler: col 
        }
    }

    pub fn solve(&mut self) -> f64 {
        self.collision_handler
    }
}