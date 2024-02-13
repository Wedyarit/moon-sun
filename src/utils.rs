use ggez::graphics::Color;

use crate::object::{Object, Team};

pub fn count_objects(objects: &Vec<Object>, team: Team) -> u32 {
    let mut count = 0;
    for object in objects {
        if object.team == team {
            count += 1;
        }
    }
    count
}

pub fn lerp_color(color1: Color, color2: Color, t: f32) -> Color {
    let r = (color1.r * (1.0 - t) + color2.r * t).round() as u8;
    let g = (color1.g * (1.0 - t) + color2.g * t).round() as u8;
    let b = (color1.b * (1.0 - t) + color2.b * t).round() as u8;
    let a = (color1.a * (1.0 - t) + color2.a * t).round() as u8;
    Color::from_rgba(r, g, b, a)
}