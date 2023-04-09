
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    types::Vector3,
    utils::{hsl_to_point, point_to_hsl}, PolineErrors,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorPointCollection {
    pub xyz: Option<Vector3>,
    pub color: Option<Vector3>,
    pub inverted_lightness: bool,
}

#[wasm_bindgen]
impl ColorPointCollection {
    pub fn new(init: JsValue) -> Self {
        serde_wasm_bindgen::from_value(init).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Vector3,
    _inverted_lightness: bool,
}

impl Default for ColorPoint {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            color: Vector3(0.0_f32, 0.0_f32, 0.0_f32),
            _inverted_lightness: false,
        }
    }
}


#[wasm_bindgen]
impl ColorPoint {
    pub fn new(initial: ColorPointCollection) -> Self {
        let mut result = Self::default();
        result._inverted_lightness = initial.inverted_lightness;
        match (initial.xyz, initial.color) {
            (Some(Vector3(x, y, z)), _) => {
                result.x = x;
                result.y = y;
                result.z = z;
                result.color = point_to_hsl(Vector3(x, y, z), initial.inverted_lightness);
                result
            }
            (_, Some(color)) => {
                result.color = color;
                let Vector3(x, y, z) = hsl_to_point(color, initial.inverted_lightness);
                result.x = x;
                result.y = y;
                result.z = z;
                result
            }
            _ => unreachable!()
        }
    }

    pub fn set_position(&mut self, new_position: Vector3) {
        let Vector3(x, y, z) = new_position;
        self.x = x;
        self.y = y;
        self.z = z;
        self.color = point_to_hsl(new_position, self._inverted_lightness)
    }

    pub fn position(&self) -> Vector3 {
        Vector3(self.x, self.y, self.z)
    }

    pub fn set_hsl(&mut self, new_color: Vector3) {
        self.color = new_color;
        let Vector3(x, y, z) = hsl_to_point(new_color, self._inverted_lightness);
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn hsl(&self) -> Vector3 {
        self.color
    }

    pub fn hsl_css(&self) -> String {
        let Vector3(h, s, l) = self.color;
        let hue = h;
        let saturation = s * 100.0;
        let luminance = l * 100.0;
        format!("hsl({hue},{saturation}%,{luminance}%")
    }

    pub fn shift_hue(&mut self, angle: f32) {
        self.color.0 = (360.0 + (self.color.0 + angle)) % 360.0;
        let Vector3(x, y, z) = hsl_to_point(self.color, self._inverted_lightness);
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_color_from_point() {
        let color_point = ColorPoint::new(ColorPointCollection {
            xyz: Some(Vector3(1.0, 1.0, 1.0)),
            color: None,
            inverted_lightness: true,
        });
        assert_eq!(color_point.color, Vector3(
            45.0,
        1.0,
        -0.41421354
        )
        );
    }

        #[test]
    fn generate_point_from_color() {
        let color_point = ColorPoint::new(ColorPointCollection {
            xyz: None,
            color: Some(Vector3(1.0, 1.0, 1.0)),
            inverted_lightness: true,
        });
        assert_eq!(color_point.position(), Vector3(
           0.5,0.5,1.0
        )
        );
    }
}
