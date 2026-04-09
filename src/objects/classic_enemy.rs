use serde::Deserialize;
use crate::functionals::math::Vec2;

#[derive(Debug, Deserialize)]
pub struct ClassicEnemy
{
    health : f32,
    speed : f32,
    position: Vec2,
    dir : Direction,
    size: Vec2,
}

#[derive(Debug, Deserialize)]
enum Direction {
    Left,
    Right,
}
impl ClassicEnemy
{
    pub fn r#move(&mut self, width : f32)
    {
        if let Direction::Right = self.dir
        {
            self.position.x = self.position.x + self.speed;
            if self.position.x > width + self.size.x
            {
                self.position.x = width + self.size.x;
                self.dir = Direction::Left;
                self.position.y += crate::constants::ENEMY_LINE_HEIGHT;
            }
        }
        if let Direction::Left = self.dir
        {
            self.position.x = self.position.x - self.speed;
            if self.position.x < 0.0
            {
                self.position.x = 0.0;
                self.dir = Direction::Right;
                self.position.y += crate::constants::ENEMY_LINE_HEIGHT;
            }
        }
    }

    ///Allows you to deal damage to this enemy.
    ///
    ///Returns true if enemy is killed, and false if still alive.
    pub fn take_damage(&mut self, damage : f32) -> bool
    {
        self.health -= damage;
        if self.health < 0.0 {
            true
        }
        else
        {
            false
        }

    }

}