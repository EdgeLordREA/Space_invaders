use crate::objects::player::Player;
use crate::UI::button::Button;

pub struct Upgrade {
    button: Button,
    name: String,
    cost: i32,
    upgrade : i32
}

impl Upgrade {
    pub fn do_upgrade(&self, player : &mut Player)
    {
        
    }
}