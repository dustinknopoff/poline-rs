
use std::ops::Index;

use color_point::ColorPoint;
use decorum::R32;
use types::{PartialVector3, Vector3};
use utils::{distance, optional_vector3, vectors_on_line};

use crate::color_point::ColorPointCollection;

pub(crate) mod color_point;
pub(crate) mod positions;
pub(crate) mod types;
pub(crate) mod utils;

pub use positions::PositionScale;
pub use utils::random_hsl_pair;
pub use utils::random_hsl_triple;

#[derive(thiserror::Error, Debug)]
pub enum PolineErrors {
    #[error("At least one is required")]
    MissingArgument,
    #[error("Point not found")]
    PointNotFound,
}

pub struct PolineOptions {
    anchor_colors: Option<Vec<Vector3>>,
    num_points: usize,
    position_function: PositionScale,
    position_function_x: Option<PositionScale>,
    position_function_y: Option<PositionScale>,
    position_function_z: Option<PositionScale>,
    inverted_lightness: bool,
    closed_loop: bool,
}

impl Default for PolineOptions {
    fn default() -> Self {
        Self {
            anchor_colors: Some(random_hsl_pair(None, None, None)),
            num_points: 4,
            position_function: PositionScale::Sinusoidal,
            position_function_x: None,
            position_function_y: None,
            position_function_z: None,
            inverted_lightness: false,
            closed_loop: false,
        }
    }
}

pub struct Poline {
    #[allow(dead_code)]
    needs_update: bool,
    anchor_points: Vec<ColorPoint>,
    num_points: usize,
    points: Vec<Vec<ColorPoint>>,
    position_function_x: PositionScale,
    position_function_y: PositionScale,
    position_function_z: PositionScale,
    anchor_pairs: Vec<(ColorPoint, ColorPoint)>,
    connect_last_and_first_anchor: bool,
    #[allow(dead_code)]
    animation_frame: Option<f32>,
    inverted_lightness: bool,
}

impl Poline {
    pub fn new(options: PolineOptions) -> Self {
        let anchor_colors = options
            .anchor_colors
            .unwrap_or(random_hsl_pair(None, None, None));
        assert!(anchor_colors.len() >= 2);
        let anchor_points: Vec<ColorPoint> = anchor_colors
            .into_iter()
            .map(|point| {
                ColorPoint::new(ColorPointCollection {
                    xyz: None,
                    color: Some(point),
                    inverted_lightness: options.inverted_lightness,
                }).expect("We're guranteeing we have xyz or color")
            })
            .collect();
        let num_points = options.num_points + 2;
        let position_function_x = options
            .position_function_x
            .unwrap_or(options.position_function);
        let position_function_y = options
            .position_function_y
            .unwrap_or(options.position_function);
        let position_function_z = options
            .position_function_z
            .unwrap_or(options.position_function);
        let (anchor_pairs, points) = Self::_update_anchor_pairs(
            options.closed_loop,
            anchor_points.clone(),
            options.inverted_lightness,
            options.num_points,
            position_function_x,
            position_function_y,
            position_function_z,
        );
        Self {
            anchor_points,
            num_points,
            position_function_x,
            position_function_y,
            position_function_z,
            connect_last_and_first_anchor: options.closed_loop,
            inverted_lightness: options.inverted_lightness,
            needs_update: true,
            anchor_pairs,
            animation_frame: None,
            points,
        }
    }

    fn _update_anchor_pairs(
        _loop: bool,
        anchor_points: Vec<ColorPoint>,
        inverted_lightness: bool,
        num_points: usize,
        fx: PositionScale,
        fy: PositionScale,
        fz: PositionScale,
    ) -> (Vec<(ColorPoint, ColorPoint)>, Vec<Vec<ColorPoint>>) {
        let mut anchor_pairs = Vec::with_capacity(anchor_points.len());
        let anchor_points_length = if _loop {
            anchor_points.len()
        } else {
            anchor_points.len() - 1
        };

        for i in 0..anchor_points_length {
            anchor_pairs.push((
                anchor_points[i],
                anchor_points[(i + 1) % anchor_points_length],
            ));
        }

        let points: Vec<Vec<ColorPoint>> = anchor_pairs
            .iter()
            .clone()
            .enumerate()
            .map(|(idx, pair)| {
                let p1_position = pair.0.position();
                let p2_position = pair.1.position();
                vectors_on_line(
                    p1_position,
                    p2_position,
                    Some(num_points),
                    idx % 2 == 0,
                    Some(fx),
                    Some(fy),
                    Some(fz),
                )
                .into_iter()
                .map(|point| {
                    ColorPoint::new(ColorPointCollection {
                        xyz: Some(point),
                        color: None,
                        inverted_lightness,
                    }).expect("We're guranteeing we have xyz or color")
                })
                .collect()
            })
            .collect();
        (anchor_pairs, points)
    }

    pub fn update_anchor_pairs(&mut self) {
        let (anchor_pairs, points) = Self::_update_anchor_pairs(
            self.connect_last_and_first_anchor,
            self.anchor_points.clone(),
            self.inverted_lightness,
            self.num_points,
            self.position_function_x,
            self.position_function_y,
            self.position_function_z,
        );
        self.anchor_pairs = anchor_pairs;
        self.points = points;
    }

    pub fn add_anchor_point(
        &mut self,
        initial: ColorPointCollection,
        insert_at_index: Option<usize>,
    ) -> ColorPoint {
        let new_anchor = ColorPoint::new(initial).expect("We're guranteeing we have xyz or color");
        if let Some(index) = insert_at_index {
            self.anchor_points.insert(index, new_anchor);
        } else {
            self.anchor_points.push(new_anchor)
        };
        self.update_anchor_pairs();
        new_anchor
    }

    pub fn remove_anchor_point_at_index(&mut self, index: usize) {
        self.anchor_points.remove(index);
        self.update_anchor_pairs();
    }

    pub fn remove_anchor_point(&mut self, point: ColorPoint) -> Result<(),PolineErrors> {
        let index = self.anchor_points.iter().position(|&p| p == point);
        if let Some(index) = index {
            self.remove_anchor_point_at_index(index);
            Ok(())
        } else {
            Err(PolineErrors::PointNotFound)
        }
    }

    pub fn update_anchor_point_at_index(
        &mut self,
        index: usize,
        initial: ColorPointCollection,
    ) -> ColorPoint {
        let mut point = self.anchor_points[index];
        if let Some(xyz) = initial.xyz {
            point.set_position(xyz);
        };
        if let Some(color) = initial.color {
            point.set_hsl(color);
        };
        self.update_anchor_pairs();

        point
    }

    pub fn update_anchor_point(&mut self, point: ColorPoint, initial: ColorPointCollection) -> Result<(), PolineErrors> {
        let index = self.anchor_points.iter().position(|&p| p == point);
        if let Some(index) = index {
            self.update_anchor_point_at_index(index, initial);
            Ok(())
        } else {
            Err(PolineErrors::PointNotFound)
        }
    }

    pub fn get_closest_anchor_point(
        &self,
        xyz: PartialVector3,
        max_distance: f32,
    ) -> Option<ColorPoint> {
        let distances: Vec<R32> = self
            .anchor_points
            .iter()
            .map(|anchor| {
                R32::try_from(distance(optional_vector3(anchor.position()), xyz, false)).unwrap()
            })
            .collect();
        let min_distance: &R32 = distances.iter().min().unwrap();
        if min_distance > &R32::from(max_distance) {
            return None;
        }

        let closest_anchor = distances.iter().position(|p| p == min_distance);
        closest_anchor.map(|index| *self.anchor_points.index(index))
    }

    pub fn shift_hue(&mut self, shift: f32) {
        self.anchor_points
            .iter_mut()
            .for_each(|point| point.shift_hue(shift));
        self.update_anchor_pairs();
    }
}

impl Poline {
    pub fn flattened_points(&self) -> Vec<ColorPoint> {
        self.points
            .clone()
            .into_iter()
            .flatten()
            .enumerate()
            .filter(|(idx, _)| {
                if idx != &0 {
                    idx % self.num_points == 0
                } else {
                    true
                }
            })
            .map(|(_, elem)| elem)
            .collect()
    }

    pub fn colors(&self) -> Vec<Vector3> {
        let colors: Vec<Vector3> = self
            .flattened_points()
            .iter()
            .map(|point| point.color)
            .collect();

        if self.connect_last_and_first_anchor {
            colors.split_last().unwrap().1.to_vec()
        } else {
            colors
        }
    }

    pub fn colors_css(&self) -> Vec<String> {
        let colors: Vec<String> = self
            .flattened_points()
            .iter()
            .map(|point| point.hsl_css())
            .collect();

        if self.connect_last_and_first_anchor {
            colors.split_last().unwrap().1.to_vec()
        } else {
            colors
        }
    }
}
