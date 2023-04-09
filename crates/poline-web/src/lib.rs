use poline_core::{Poline, PolineOptions, position_from_scale, number_as_enum, PolineErrors, PositionScale, random_hsl_pair};
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, throw_str};



#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PolineJsOptions {
    num_points: usize,
    position_function: usize,
    position_function_x: Option<usize>,
    position_function_y: Option<usize>,
    position_function_z: Option<usize>,
    inverted_lightness: bool,
    closed_loop: bool,
}

impl PolineJsOptions {
    pub fn as_rs_options(self) -> PolineOptions {
        let mut options = PolineOptions::default();
        options.num_points = self.num_points;
        options.position_function = number_as_enum(self.position_function);
        options.position_function_x = self.position_function_x.map(number_as_enum);
        options.position_function_y = self.position_function_y.map(number_as_enum);
        options.position_function_z = self.position_function_z.map(number_as_enum);
        options
    }
}

#[wasm_bindgen]
pub fn poline(options: JsValue) -> Poline {
    let parsed_values = serde_wasm_bindgen::from_value::<PolineJsOptions>(options);
    match parsed_values {
        Err(err) => {
            eprintln!("{err}");
            throw_str(&format!("{err}"))
        }
        Ok(options) => {
            Poline::from(options.as_rs_options())
        }
    }
}

