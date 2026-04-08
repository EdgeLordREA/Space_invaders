use crate::objects::*;
pub struct GameState
{
    player : player::Player,
    bullets : Vec<bullet::Bullet>,
    enemies : Vec<enemy::Enemy>

}

impl GameState {
    pub(crate) fn new(player : player::Player) -> GameState {
        let bullets : Vec<bullet::Bullet> = Vec::new();
        let enemies : Vec<enemy::Enemy> = Vec::new();
        GameState{
            player,
            bullets,
            enemies
        }
    }
}