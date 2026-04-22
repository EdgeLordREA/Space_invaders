use serde::{Deserialize, Serialize};
use crate::objects::player::{Player, PlayerStat};
use crate::UI::button::Button;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Upgrade {
    pub button: Button,
    pub player_stat: PlayerStat,
    pub cost: i32,
    pub upgrade : f32
}

impl Upgrade {
    pub fn do_upgrade(&self, player : &mut Player)
    {
        
    }
}