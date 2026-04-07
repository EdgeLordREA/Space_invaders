struct Player
{
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    atk_speed: f32,
    atk_cooldown : f32,
    dmg : f32
}
impl Player
{
    pub fn can_attack(self) -> bool
        {
            self.atk_cooldown < 0.0
        }

    pub fn attack(&mut self)
    {
        self.atk_cooldown = 1.0 / self.atk_speed
    }
}