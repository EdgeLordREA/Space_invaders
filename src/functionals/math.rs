use serde::Deserialize;

pub fn between(left: f32, right: f32, top: f32, bottom: f32, loc : Vec2) -> bool {
    let horizontal = loc.x >= left && loc.x < right;
    let vertical = loc.y >= top && loc.y < bottom;
    horizontal && vertical
}

#[derive(Debug, Deserialize)]
pub struct Vec2
{
    pub x : f32,
    pub y : f32,
}