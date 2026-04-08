use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};

pub struct Player
{
    position : Vec2,
    speed: f32,
    atk_speed: f32,
    atk_cooldown : f32,
    dmg : f32
}
impl Player
{
    pub fn can_attack(&self) -> bool
        {
            self.atk_cooldown < 0.0
        }

    pub fn attack(&mut self)
    {
        self.atk_cooldown = 1.0 / self.atk_speed
    }

    pub const fn new(screen_width : f32, screen_height : f32) -> Player {
        Player {
            position: Vec2::new(screen_width/2.0, screen_height-10.0), // Start at the origin
            speed: 200.0,                  // Base movement speed
            atk_speed: 1.0,                // 1 attack per second
            atk_cooldown: 0.0,             // Ready to attack immediately
            dmg: 10.0,                     // Base damage
        }
    }
}