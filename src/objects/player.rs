use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};
use crate::gamestate::GameState;
use crate::objects::bullet::{Bullet, Source};

pub struct Player
{
    pub position : Vec2,
    pub dimensions : Vec2,
    pub speed: f32,
    atk_speed: f32,
    pub atk_cooldown : f32,
    bullet_speed : Vec2,
    bullet_size : Vec2,
    bullet_damage: f32,
}
impl Player
{
    pub fn can_attack(&self) -> bool
        {
            self.atk_cooldown < 0.0
        }

    pub fn attack(&mut self,  bullets : &mut Vec<Bullet>)
    {
        self.atk_cooldown = 1.0 / self.atk_speed;
        let damage = self.calculate_damage();
        let x_offset = self.dimensions / 2.0;
        let bullet = Bullet::new(self.position + x_offset,self.bullet_size, self.bullet_speed, Source::Player, damage);
        bullets.push(bullet);
        self.bullet_size *= 1.5;
    }

    pub fn calculate_damage(&self) -> f32
    {
        self.bullet_damage
    }
    pub fn run_player(&mut self, delta : f32)
    {
        self.atk_cooldown -= delta;
    }

    pub fn new(screen_width : f32, screen_height : f32) -> Player {
        Player {
            position: Vec2::new(screen_width/2.0, screen_height-10.0), // Start at the origin
            dimensions: Vec2::new(30.0, 20.0), //size of player
            speed: 200.0,                  // Base movement speed
            atk_speed: 8.0,                // 1 attack per second
            atk_cooldown: 0.0,             // Ready to attack immediately
            bullet_damage: 10.0,                     // Base damage
            bullet_speed: Vec2::new(0.0, -100.0), // speed and direction of bullet
            bullet_size: Vec2::new(1.0, 1.0), // size of bullet
        }
    }
    pub fn position(&self) -> Vec2
    {
        self.position
    }
}