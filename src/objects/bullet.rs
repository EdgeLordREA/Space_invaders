use macroquad::math::{Vec2};

pub struct Bullet {
    position : Vec2,
    pub dimensions : Vec2,
    speed: Vec2,
    source: Source,
    damage : f32
}

impl Bullet {
    pub fn new(position : Vec2, dimensions : Vec2, speed: Vec2, source: Source, damage : f32) -> Bullet {
        Bullet{
            position,
            dimensions,
            speed,
            source,
            damage
        }
    }    
    pub fn move_bullet(&mut self, delta : f32)
    {
        self.position += self.speed * delta;
    }
    
    pub fn get_position(&self) -> Vec2 {
        self.position
    }
}

pub enum Source{
    Player,
    Enemy
}