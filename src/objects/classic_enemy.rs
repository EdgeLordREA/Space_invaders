
/// Classic Enemy module.
/// 
/// Contains the ClassicEnemy struct representing a basic enemy type
/// with movement patterns and health management.

use serde::{Deserialize, Serialize};
use crate::functionals::rect::Rect;
use crate::functionals::vec2::Vec2;

/// A basic enemy type that moves in a bouncing pattern.
/// 
/// ClassicEnemy moves horizontally and drops down when hitting screen edges.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClassicEnemy
{
    /// Current health of the enemy
    pub(crate) health : f32,
    /// Movement speed in pixels per second
    speed : f32,
    /// Current position in world coordinates
    pub shape : Rect,
    /// Current horizontal movement direction
    dir : Direction,
    pub cash_value : i32,
    pub instance : i32
}

/// Horizontal movement direction for enemies.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Direction {
    /// Moving towards the left side of the screen
    Left,
    /// Moving towards the right side of the screen
    Right,
}

impl From<i8> for Direction
{
    fn from(i : i8) -> Direction
    {
        match i
        {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Invalid direction value: {}", i)
        }
    }
}

impl Into<i8> for Direction
{
    fn into(self) -> i8
    {
        match self
        {
            Direction::Left => 0,
            Direction::Right => 1,
        }
    }
}
impl ClassicEnemy
{
    pub fn new(health : f32, speed : f32, position : Vec2, size : Vec2, direction: Direction, cash_value : i32, instance : i32) -> ClassicEnemy
    {
        ClassicEnemy{
            health,
            speed,
            shape : Rect::new(position.x, position.y, size.x, size.y),
            dir : direction,
            cash_value,
            instance
        }
    }

    pub fn deserialize(health : f32, speed : f32, posx : f32, posy : f32, width : f32, height : f32, direction: i8, cash_value : i32) -> ClassicEnemy
    {
        ClassicEnemy{
            health,
            speed,
            shape: Rect { x: posx, y: posy, w: width, h: height},
            dir: direction.into(),
            cash_value,
            instance: 0,
        }
    }

    pub fn serialize(&self) -> String
    {
        format!("{},{},{},{},{},{},{},{}",
                self.health,
                self.speed,
                self.shape.x,
                self.shape.y,
                self.shape.w,
                self.shape.h,
                self.dir as i8,
                self.cash_value)
    }

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
            self.shape.x = self.shape.x + self.speed;
            if self.shape.x > width - self.shape.w
            {
                self.shape.x = width - self.shape.w;
                self.dir = Direction::Left;
                self.shape.y += crate::constants::ENEMY_LINE_HEIGHT;
            }
        }
        if let Direction::Left = self.dir
        {
            self.shape.x = self.shape.x - self.speed;
            if self.shape.x < 0.0
            {
                self.shape.x = 0.0;
                self.dir = Direction::Right;
                self.shape.y += crate::constants::ENEMY_LINE_HEIGHT;
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