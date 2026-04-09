use macroquad::math::Vec2;

pub fn between(left: f32, right: f32, top: f32, bottom: f32, loc : Vec2) -> bool {
    let horizontal = loc.x >= left && loc.x < right;
    let vertical = loc.y >= top && loc.y < bottom;
    horizontal && vertical
}