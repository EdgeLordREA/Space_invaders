use std::ops::{Deref, DerefMut};
use crate::objects::*;
use crate::objects::player::Player;

pub struct GameState
{
    pub player : player::Player,
    pub bullets : Vec<bullet::Bullet>,
    pub enemies : Vec<enemy::Enemy>

}

impl GameState {
    pub fn new(screen_width : f32, screen_height : f32) -> GameState {
        let player = Player::new(screen_width, screen_height);
        let bullets : Vec<bullet::Bullet> = Vec::new();
        let enemies : Vec<enemy::Enemy> = Vec::new();
        GameState{
            player,
            bullets,
            enemies
        }
    }

    pub fn run_state(&mut self, delta : f32)
    {
        
        for bullet in self.bullets.iter_mut(){
            bullet.move_bullet(delta);
        }
    }
}