use macroquad::math::{Vec2};

pub struct Bullet {
    position : Vec2,
    speed: Vec2,
    source: Source
}

enum Source{
    Player,
    Enemy
}