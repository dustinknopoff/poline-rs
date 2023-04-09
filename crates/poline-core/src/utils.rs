use std::f32::consts::PI;

use rand::random;

use crate::{
    positions::{PositionScale, position_from_scale},
    types::{PartialVector3, Vector2, Vector3},
};

pub fn optional_vector3(vector3: Vector3) -> PartialVector3 {
    let Vector3(x, y, z) = vector3;
    PartialVector3(Some(x), Some(y), Some(z))
}

pub fn number_as_enum(scale_num: usize) -> PositionScale {
    match scale_num {
        0 => PositionScale::Linear,
        1 => PositionScale::Exponential,
        2 => PositionScale::Cubic,
        3 => PositionScale::Quadratic,
        4 => PositionScale::Quartic,
        5 => PositionScale::Sinusoidal,
        6 => PositionScale::Asinusoidal,
        7 => PositionScale::Arc,
        8 => PositionScale::SmoothStep,
        _ => unreachable!()
    }
}

///
/// Converts the given (x, y, z) coordinate to an HSL color
/// The (x, y) values are used to calculate the hue, while the z value is used as the saturation
/// The lightness value is calculated based on the distance of (x, y) from the center (0.5, 0.5)
///
/// point_to_hsl((0.5, 0.5, 1_f32), false); // [0, 1, 0.5]
/// point_to_hsl((0.5, 0.5, 0_f32), false); // [0, 1, 0]
/// point_to_hsl((0.5, 0.5, 1_f32), false); // [0, 1, 1]
///
pub fn point_to_hsl(xyz: Vector3, inverted_lightness: bool) -> Vector3 {
    let Vector3(x, y, z) = xyz;

    // cy and cx are the center (y and x) values
    let cx = 0.5_f32;
    let cy = 0.5_f32;

    // Calculate the angle between the point (x, y) and the center (cx, cy)
    let radians = (y - cy).atan2(x - cx);

    // Convert the angle to degrees and shift it so that it goes from 0 to 360
    let mut deg = radians * (180_f32 / PI);
    deg = (360_f32 + deg) % 360_f32;

    // The saturation value is taken from the z coordinate
    let s = z;

    let dist = ((y - cy).powf(2_f32) + (x - cx).powf(2_f32)).sqrt();
    let l = dist / cx;

    let lightness = if inverted_lightness { 1_f32 - l } else { l };
    // Return the HSL color as an array [hue, saturation, lightness]
    Vector3(deg, s, lightness)
}

///
/// Converts the given HSL color to an (x, y, z) coordinate
/// The hue value is used to calculate the (x, y) position, while the saturation value is used as the z coordinate
/// The lightness value is used to calculate the distance from the center (0.5, 0.5)
/// @example
/// hslToPoint([0, 1, 0.5]) // [0.5, 0.5, 1]
/// hslToPoint([0, 1, 0]) // [0.5, 0.5, 1]
/// hslToPoint([0, 1, 1]) // [0.5, 0.5, 1]
/// hslToPoint([0, 0, 0.5]) // [0.5, 0.5, 0]
///
pub fn hsl_to_point(hsl: Vector3, inverted_lightness: bool) -> Vector3 {
    // Destructure the input array into separate hue, saturation, and lightness values
    let Vector3(h, s, l) = hsl;
    // cx and cy are the center (x and y) values
    let cx = 0.5;
    let cy = 0.5;
    // Calculate the angle in radians based on the hue value
    let radians = h / (180.0 / PI);

    // Calculate the distance from the center based on the lightness value
    let dist = if inverted_lightness {
        (1.0 - l) * cx
    } else {
        1.0 * cx
    };

    // Calculate the x and y coordinates based on the distance and angle
    let x = cx + dist * radians.cos();
    let y = cy + dist * radians.sin();
    // The z coordinate is equal to the saturation value
    let z = s;
    // Return the (x, y, z) coordinate as an array [x, y, z]
    Vector3(x, y, z)
}

pub fn random_hsl_pair(
    start_hue: Option<f32>,
    saturations: Option<Vector2>,
    lightnesses: Option<Vector2>,
) -> Vec<Vector3> {
    let start_hue = start_hue.unwrap_or(random::<f32>() * 360.0);
    let saturations = saturations.unwrap_or(Vector2(random(), random()));
    let lightnesses = lightnesses.unwrap_or(Vector2(
        0.75 + random::<f32>() * 0.2,
        0.3 + random::<f32>() * 0.2,
    ));
    vec![
        Vector3(start_hue, saturations.0, lightnesses.0),
        Vector3(
            (start_hue + 60.0 + random::<f32>() * 180.0) % 360.0,
            saturations.1,
            lightnesses.1,
        ),
    ]
}

#[allow(dead_code)]
pub fn random_hsl_triple(
    start_hue: Option<f32>,
    saturations: Option<Vector3>,
    lightnesses: Option<Vector3>,
) -> Vec<Vector3> {
    let start_hue = start_hue.unwrap_or(random::<f32>() * 360.0);
    let saturations = saturations.unwrap_or(Vector3(random(), random(), random()));
    let lightnesses = lightnesses.unwrap_or(Vector3(
        0.75 + random::<f32>() * 0.2,
        0.3 + random::<f32>() * 0.2,
        0.75 + random::<f32>() * 0.2,
    ));
    vec![
        Vector3(start_hue, saturations.0, lightnesses.0),
        Vector3(
            (start_hue + 60.0 + random::<f32>() * 180.0) % 360.0,
            saturations.1,
            lightnesses.1,
        ),
        Vector3(
            (start_hue + 60.0 + random::<f32>() * 180.0) % 360.0,
            saturations.2,
            lightnesses.2,
        ),
    ]
}

fn _invert(number: f32, invert: bool) -> f32 {
    if invert {
        1.0 - number
    } else {
        number
    }
}

pub fn vector_on_line(
    t: f32,
    p1: Vector3,
    p2: Vector3,
    invert: bool,
    fx: Option<PositionScale>,
    fy: Option<PositionScale>,
    fz: Option<PositionScale>,
) -> Vector3 {
    let t_modified_x = if let Some(fx) = fx {
        position_from_scale(fx,t, invert)
    } else {
        _invert(t, invert)
    };
    let t_modified_y = if let Some(fy) = fy {
        position_from_scale(fy,t, invert)
    } else {
        _invert(t, invert)
    };
    let t_modified_z = if let Some(fz) = fz {
        position_from_scale(fz, t,invert)
    } else {
        _invert(t, invert)
    };

    let x = (1.0 - t_modified_x) * p1.0 + t_modified_x * p2.0;
    let y = (1.0 - t_modified_y) * p1.1 + t_modified_y * p2.1;
    let z = (1.0 - t_modified_z) * p1.2 + t_modified_z * p2.2;

    Vector3(x, y, z)
}

pub fn vectors_on_line(
    p1: Vector3,
    p2: Vector3,
    num_points: Option<usize>,
    invert: bool,
    fx: Option<PositionScale>,
    fy: Option<PositionScale>,
    fz: Option<PositionScale>,
) -> Vec<Vector3> {
    let num_points = num_points.unwrap_or(4);
    let mut points = Vec::new();

    for i in 0..num_points {
        let point = vector_on_line((i / (num_points - 1)) as f32, p1, p2, invert, fx, fy, fz);
        points.push(point);
    }

    points
}

///
/// Calculates the distance between two points
/// let p1 = (Some(0.0), Some(0.0), Some(0.0));
/// let p2 = (Some(1.0), Some(1.0), Some(1.0));
/// let dist = distance(p1, p2, false);
/// assert_eq!(distance(p1, p2, false), 1.7320508075688772);
///
pub fn distance(p1: PartialVector3, p2: PartialVector3, hue_mode: bool) -> f32 {
    let a1 = p1.0;
    let a2 = p2.0;
    let diff_a = match (a1, a2) {
        (Some(a1), Some(a2)) if hue_mode => ((a1 - a2).abs().min(360.0 - (a1 - a2).abs())) / 360.0,
        (Some(a1), Some(a2)) => a1 - a2,
        _ => 0.0,
    };

    let a = diff_a;
    let b = match (p1.1, p2.1) {
        (Some(p1), Some(p2)) => p2 - p1,
        _ => 0.0,
    };
    let c = match (p1.2, p2.2) {
        (Some(p1), Some(p2)) => p2 - p1,
        _ => 0.0,
    };

    (a * a + b * b + c * c).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{PartialVector3, Vector3},
        utils::{distance, hsl_to_point, point_to_hsl},
    };

    #[test]
    fn point_to_hsl_test() {
        assert_eq!(
            point_to_hsl(Vector3(0.5, 0.5, 1_f32), false),
            Vector3(0_f32, 1_f32, 0.0)
        );
        assert_eq!(
            point_to_hsl(Vector3(0.5, 0.5, 0_f32), false),
            Vector3(0_f32, 0_f32, 0_f32)
        );
        assert_eq!(
            point_to_hsl(Vector3(0.5, 0.5, 1_f32), false),
            Vector3(0_f32, 1_f32, 0_f32)
        );
    }

    #[test]
    fn hsl_to_point_test() {
        assert_eq!(
            hsl_to_point(Vector3(0.0, 1.0, 0.5), false),
            Vector3(1.0, 0.5, 1.0)
        );
        assert_eq!(
            hsl_to_point(Vector3(0.0, 1.0, 0.0), false),
            Vector3(1.0, 0.5, 1.0)
        );
        assert_eq!(
            hsl_to_point(Vector3(0.0, 1.0, 1.0), false),
            Vector3(1.0, 0.5, 1.0)
        );
        assert_eq!(
            hsl_to_point(Vector3(0.0, 0.0, 0.5), false),
            Vector3(1.0, 0.5, 0.0)
        );
    }

    #[test]
    fn distance_test() {
        let p1 = PartialVector3(Some(0.0), Some(0.0), Some(0.0));
        let p2 = PartialVector3(Some(1.0), Some(1.0), Some(1.0));
        assert_eq!(distance(p1, p2, false), 1.732_050_8);
    }
}
