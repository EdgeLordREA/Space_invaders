/// Classic Enemy module.
/// 
/// Contains the ClassicEnemy struct representing a basic enemy type
/// with movement patterns and health management.

use serde::Deserialize;
use crate::functionals::math::Vec2;

/// A basic enemy type that moves in a bouncing pattern.
/// 
/// ClassicEnemy moves horizontally and drops down when hitting screen edges.
#[derive(Debug, Deserialize)]
pub struct ClassicEnemy
{
    /// Current health of the enemy
    health : f32,
    /// Movement speed in pixels per second
    speed : f32,
    /// Current position in world coordinates
    position: Vec2,
    /// Current horizontal movement direction
    dir : Direction,
    /// Width and height dimensions of the enemy
    size: Vec2,
}

/// Horizontal movement direction for enemies.
#[derive(Debug, Deserialize)]
enum Direction {
    /// Moving towards the left side of the screen
    Left,
    /// Moving towards the right side of the screen
    Right,
}
impl ClassicEnemy
{
    /// Moves the enemy based on its current direction.
    /// 
    /// # Arguments
    /// * `width` - Screen width boundary for collision detection
    /// 
    /// The enemy moves horizontally until it hits a screen edge,
    /// then reverses direction and moves downward by ENEMY_LINE_HEIGHT.
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

    /// Applies damage to this enemy.
    /// 
    /// # Arguments
    /// * `damage` - Amount of damage to apply
    /// 
    /// # Returns
    /// `true` if the enemy is killed (health <= 0), `false` otherwise.
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