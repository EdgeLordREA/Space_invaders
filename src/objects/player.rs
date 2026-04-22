use crate::functionals::rect::Rect;
/// Player character module.
/// 
/// Contains the Player struct and related functionality for controlling
/// the player character, including movement, combat, and state management.

use crate::objects::bullet::{Bullet, Source};
use crate::functionals::vec2::Vec2;

/// The player-controlled character in the game.
/// 
/// Contains position, dimensions, combat stats, and cooldown tracking.
pub struct Player
{
    /// Width and height dimensions of the player
    pub shape : Rect,
    /// Movement speed in pixels per second
    pub speed: f32,
    /// Attack speed (attacks per second)
    atk_speed: f32,
    /// Current attack cooldown timer
    pub atk_cooldown : f32,
    /// Velocity vector for spawned bullets
    bullet_speed : Vec2,
    bullet_count : i32,
    /// Dimensions of spawned bullets
    bullet_radius: f32,
    /// Damage dealt by player's bullets
    bullet_damage: f32,
    bullet_pen : i32,
    pub cash : i32
}


impl Player
{
    /// Checks if the player can currently attack.
    /// 
    /// # Returns
    /// `true` if the attack cooldown has expired, `false` otherwise.
    pub fn can_attack(&self) -> bool
        {
            self.atk_cooldown < 0.0
        }

    /// Performs an attack, creating a new bullet.
    /// 
    /// # Arguments
    /// * `bullets` - Mutable reference to the vector of active bullets
    /// 
    /// Sets the attack cooldown and spawns a bullet from the player's center position.
    pub fn attack(&mut self, bullets: &mut Vec<Bullet>) {
        self.atk_cooldown = 1.0 / self.atk_speed;
        let damage = self.calculate_damage();
        let center = self.shape.center();
        let bullet_spacing = self.bullet_radius * 2.0;
        // Total width of the bullet line
        let total_width = (self.bullet_count.saturating_sub(1) as f32) * bullet_spacing;
        // Start position so the line is centered: center.x - half_width
        let start_x = center.x - total_width / 2.0;
        let middle = (self.bullet_count - 1) as f32 / 2.0;
        for i in 0..self.bullet_count {
            let y_offset = (i as f32 - middle).abs() * 10.0;
            let x = start_x + (i as f32) * bullet_spacing;
            let y = center.y - 20.0 + y_offset; // Same row as player center
            let origin = Vec2::new(x, y);

            let bullet = Bullet::new(
                origin,
                self.bullet_radius,
                self.bullet_speed,
                Source::Player,
                damage,
                self.bullet_pen
            );
            bullets.push(bullet);
        }
    }

    /// Calculates the damage for the player's attack.
    /// 
    /// # Returns
    /// The base bullet damage value.
    pub fn calculate_damage(&self) -> f32
    {
        self.bullet_damage
    }
    
    /// Updates the player's internal state for the current frame.
    /// 
    /// # Arguments
    /// * `delta` - Time elapsed since the last frame
    /// 
    /// Decrements the attack cooldown timer.
    pub fn run_player(&mut self, delta : f32)
    {
        self.atk_cooldown -= delta;
    }

    /// Creates a new Player with default values.
    /// 
    /// # Arguments
    /// * `screen_width` - Width of the game screen for initial positioning
    /// * `screen_height` - Height of the game screen for initial positioning
    /// 
    /// # Returns
    /// A new Player positioned at the bottom center of the screen with default stats.
    pub fn new(screen_width : f32, screen_height : f32) -> Player {
        Player {
            shape : Rect::new(screen_width/2.0, screen_height-20.0, 30.0, 20.0), //size of player
            speed: 200.0,                  // Base movement speed
            atk_speed: 8.0,                // 1 attack per second
            atk_cooldown: 0.0,             // Ready to attack immediately
            bullet_damage: 10.0,                     // Base damage
            bullet_speed: Vec2::new(0.0, -100.0), // speed and direction of bullet
            bullet_count: 1,
            bullet_radius: 4.0, // size of bullet
            bullet_pen : 1, //number of enemies the bullet is allowed to hit
            cash : 0
        }
    }
    
    /// Gets the current position of the player.
    /// 
    /// # Returns
    /// A copy of the player's current position vector.
    pub fn position(&self) -> Vec2
    {
        Vec2::new(self.shape.x, self.shape.y)
    }

    pub fn buy_stat(&self, stat : PlayerStat, cost : i32, value : f32) -> bool
    {
        self.cash >= cost
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerStat{
    Health,
    AttackSpeed,
    BulletDamage,
    BulletSpeed,
    BulletRadius,
    BulletCount,
    BulletPen,
}