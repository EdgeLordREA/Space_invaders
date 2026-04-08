use macroquad::math::{Vec2};
use macroquad::prelude::pop_camera_state;

pub struct Bullet {
    position : Vec2,
    speed: Vec2,
    source: Source
}

impl Bullet {
    pub fn new(position : Vec2, speed: Vec2, source: Source) -> Bullet {
        Bullet{
            position,
            speed,
            source,
        }
    }    
    pub fn move_bullet(&mut self, delta : f32)
    {
        self.position += self.speed * delta;
    }
}

enum Source{
    Player,
    Enemy
}