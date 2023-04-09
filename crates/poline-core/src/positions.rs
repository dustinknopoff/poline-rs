use std::f32::consts::PI;


#[derive(Debug, Clone, Copy)]
/// Defines all possible scale function types for use in color generator
pub enum PositionScale {
    Linear,
    Exponential,
    Cubic,
    Quadratic,
    Quartic,
    Sinusoidal,
    Asinusoidal,
    Arc,
    SmoothStep,
}

impl PositionScale {
    /// Given a position on a x,y, or z calculate the position based the scale method
    pub fn position(self, t: f32, reverse: bool) -> f32 {
        use PositionScale::*;
        match self {
            Linear => t,
            Exponential => {
                if reverse {
                    1.0 - (1.0 - t).powf(2.0)
                } else {
                    t.powf(2.0)
                }
            }
            Cubic => {
                if reverse {
                    1.0 - (1.0 - t).powf(3.0)
                } else {
                    t.powf(3.0)
                }
            }
            Quadratic => {
                if reverse {
                    1.0 - (1.0 - t).powf(4.0)
                } else {
                    t.powf(4.0)
                }
            }
            Quartic => {
                if reverse {
                    1.0 - (1.0 - t).powf(5.0)
                } else {
                    t.powf(5.0)
                }
            }
            Sinusoidal => {
                if reverse {
                    1.0 - (((1.0 - t) * PI) / 2.0).sin()
                } else {
                    ((t * PI) / 2.0).sin()
                }
            }
            Asinusoidal => {
                if reverse {
                    1.0 - (1.0 - t).asin() / (PI / 2.0)
                } else {
                    t.asin() / (PI / 2.0)
                }
            }
            Arc => {
                if reverse {
                    (1.0 - (1.0 - t).powf(2.0)).sqrt()
                } else {
                    1.0 - (1.0 - t).sqrt()
                }
            }
            SmoothStep => t.powf(2.0 * (3.0 - 2.0 * t)),
        }
    }
}

pub fn position_from_scale(scale: PositionScale, t: f32, reverse: bool) -> f32 {
     use PositionScale::*;
        match scale {
            Linear => t,
            Exponential => {
                if reverse {
                    1.0 - (1.0 - t).powf(2.0)
                } else {
                    t.powf(2.0)
                }
            }
            Cubic => {
                if reverse {
                    1.0 - (1.0 - t).powf(3.0)
                } else {
                    t.powf(3.0)
                }
            }
            Quadratic => {
                if reverse {
                    1.0 - (1.0 - t).powf(4.0)
                } else {
                    t.powf(4.0)
                }
            }
            Quartic => {
                if reverse {
                    1.0 - (1.0 - t).powf(5.0)
                } else {
                    t.powf(5.0)
                }
            }
            Sinusoidal => {
                if reverse {
                    1.0 - (((1.0 - t) * PI) / 2.0).sin()
                } else {
                    ((t * PI) / 2.0).sin()
                }
            }
            Asinusoidal => {
                if reverse {
                    1.0 - (1.0 - t).asin() / (PI / 2.0)
                } else {
                    t.asin() / (PI / 2.0)
                }
            }
            Arc => {
                if reverse {
                    (1.0 - (1.0 - t).powf(2.0)).sqrt()
                } else {
                    1.0 - (1.0 - t).sqrt()
                }
            }
            SmoothStep => t.powf(2.0 * (3.0 - 2.0 * t)),
        }
    }