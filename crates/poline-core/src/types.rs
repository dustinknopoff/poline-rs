use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector2(pub f32, pub f32);
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3(pub f32, pub f32, pub f32);
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PartialVector3(pub Option<f32>, pub Option<f32>, pub Option<f32>);

#[wasm_bindgen]
impl PartialVector3 {
    pub fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self {
        Self(x, y, z)
    }
}
