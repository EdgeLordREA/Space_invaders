use crate::config::Config;
use crate::math;
use crate::objects::*;
use crate::objects::player::Player;

pub struct GameState
{
    pub player : player::Player,
    pub bullets : Vec<bullet::Bullet>,
    pub enemies : Vec<enemy::Enemy>,
    pub config : Config,
}

impl GameState {
    pub fn new(screen_width : f32, screen_height : f32) -> GameState {
        let player = Player::new(screen_width, screen_height);
        let bullets : Vec<bullet::Bullet> = Vec::new();
        let enemies : Vec<enemy::Enemy> = Vec::new();
        let config = Config{screen_width, screen_height};
        GameState{
            player,
            bullets,
            enemies,
            config,
        }
    }

    pub fn run_state(&mut self, delta : f32)
    {
        for bullet in self.bullets.iter_mut(){
            bullet.move_bullet(delta);
        }
        self.bullets.retain(|b| math::between(0.0, self.config.screen_width, 0.0, self.config.screen_height, b.get_position()));
        self.player.run_player(delta);
    }
}