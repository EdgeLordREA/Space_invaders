use crate::functionals::config::Config;
use crate::functionals::math;
use crate::functionals::vec2::Vec2;
use crate::objects::*;
use crate::objects::classic_enemy::{ClassicEnemy, Direction};
use crate::objects::player::Player;
use crate::objects::wave::Wave;

/// Represents the current state of the game.
/// 
/// Contains all active game entities and configuration settings.
pub struct GameState
{
    /// The player character controlled by the user
    pub player : player::Player,
    /// All active bullets in the game world
    pub bullets : Vec<bullet::Bullet>,
    /// All active enemies in the game world
    pub enemies : Vec<ClassicEnemy>,
    /// Game configuration including screen dimensions
    pub config : Config,
    pub wave : Wave,
    enemy_instance_count: i32,
    pub cash : i32
}

impl GameState {
    /// Creates a new GameState with initialized entities.
    /// 
    /// # Arguments
    /// * `screen_width` - Width of the game window in pixels
    /// * `screen_height` - Height of the game window in pixels
    /// 
    /// # Returns
    /// A new GameState instance with a player positioned at the bottom center,
    /// empty bullet and enemy collections, and default configuration.
    pub fn new(screen_width : f32, screen_height : f32) -> GameState {
        let bullets : Vec<bullet::Bullet> = Vec::new();
        let enemies : Vec<ClassicEnemy> = Vec::new();
        let wave = Wave::new();
        let config = Config{screen_width, screen_height};
        let player = Player::new(config.screen_width, config.screen_height);
        let enemy_instance_count = 0;
        let cash = 0;
        let mut gs = GameState{
            player,
            bullets,
            enemies,
            wave,
            config,
            enemy_instance_count,
            cash
        };
        gs.load_enemies();
        gs
    }
    
    pub fn load_enemies(&mut self)
    {
        for _ in 0..5 {

            let enemy = ClassicEnemy::new(50.0, 1.2, Vec2::new(100.0, 100.0), Vec2::new(40.0, 20.0), Direction::Right, 10, self.enemy_instance_count);
            self.wave.add_enemies(enemy);
            self.enemy_instance_count += 1;
        }
        self.wave.set_max_enemies(5);
        self.wave.set_spawn_rate(1.0);
        self.wave.set_max_duration(999.0);
    }

    /// Updates all game entities for the current frame.
    /// 
    /// # Arguments
    /// * `delta` - Time elapsed since the last frame
    /// 
    /// Performs the following updates:
    /// - Moves all bullets according to their velocity
    /// - Removes bullets that have left the screen bounds
    /// - Updates the player's internal state (e.g., cooldowns)
    pub fn run_state(&mut self, delta : f32)
    {
        self.run_bullets(delta);

        self.run_enemies(delta);

        self.player.run_player(delta);
    }

    pub fn wave_complete(&self) -> bool
    {
        self.enemies.is_empty() && self.wave.is_complete()
    }

    fn run_enemies(&mut self, delta : f32) {
        let x = self.wave.run_wave(delta, self.enemies.len() as i32);
        if x.is_some() {
            self.enemies.push(x.unwrap());
        }
        for enemy in self.enemies.iter_mut() {
            if enemy.health <= 0.0 {
                self.cash += enemy.cash_value;
                continue;
            }
            enemy.r#move(self.config.screen_width);
        }
        self.enemies.retain(|e| e.health > 0.0);
    }

    fn run_bullets(&mut self, delta: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.move_bullet(delta);
            for enemy in self.enemies.iter_mut() {
                if bullet.collides_with_rect(enemy.shape)
                {
                    if !bullet.has_hit(enemy.instance) {
                        bullet.register_hit(enemy.instance);
                        enemy.take_damage(bullet.damage);
                        break;
                    }
                }
            }
        }
        self.bullets.retain(|b| math::between(0.0, self.config.screen_width, 0.0, self.config.screen_height, b.get_position()) && b.penetration > 0);
    }
}