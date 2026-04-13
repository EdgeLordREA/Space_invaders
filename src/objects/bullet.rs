/// Bullet module.
/// 
/// Contains the Bullet struct and related types for managing
/// projectiles fired by players and enemies.

use std::ops::Mul;
use macroquad::math::Rect;
use crate::functionals::vec2::Vec2;

/// A projectile entity in the game world.
/// 
/// Represents a bullet with position, radius, velocity, source, and damage.
pub struct Bullet {
    /// Current position of the bullet
    position : Vec2,
    /// Radius of the bullet
    radius : f32,
    /// Velocity vector determining movement direction and speed
    speed: Vec2,
    /// The source that fired this bullet (player or enemy)
    source: Source,
    /// Damage value dealt by this bullet
    damage : f32
}

impl Bullet {
    /// Creates a new Bullet instance.
    /// 
    /// # Arguments
    /// * `position` - Initial position of the bullet
    /// * `radius` - Radius of the bullet
    /// * `speed` - Velocity vector for movement
    /// * `source` - Entity that fired the bullet
    /// * `damage` - Damage value to deal on hit
    pub fn new(position : Vec2, radius : f32, speed: Vec2, source: Source, damage : f32) -> Bullet {
        Bullet{
            position,
            radius,
            speed,
            source,
            damage
        }
    }    
    
    /// Updates the bullet's position based on its velocity.
    /// 
    /// # Arguments
    /// * `delta` - Time elapsed since the last frame
    pub fn move_bullet(&mut self, delta : f32)
    {
        self.position += self.speed * delta;
    }
    
    /// Gets the current position of the bullet.
    /// 
    /// # Returns
    /// A copy of the bullet's current position vector.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn collides_with_rect(self, rect: Rect) -> bool
    {
        let closestpoint = self.
    }
}

/// Identifies the source of a bullet.
/// 
/// Used to distinguish between player-fired and enemy-fired projectiles.
pub enum Source{
    /// Bullet fired by the player
    Player,
    /// Bullet fired by an enemy
    Enemy
}