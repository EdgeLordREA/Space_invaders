/// Mathematical utilities and vector operations.
/// 
/// This module provides mathematical helper functions and a 2D vector type
/// for game calculations including collision detection, movement, and physics.

use serde::Deserialize;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// Checks if a 2D point is within the specified rectangular bounds.
/// 
/// # Arguments
/// * `left` - Left boundary of the rectangle
/// * `right` - Right boundary of the rectangle
/// * `top` - Top boundary of the rectangle
/// * `bottom` - Bottom boundary of the rectangle
/// * `loc` - The 2D point to check
/// 
/// # Returns
/// `true` if the point is within bounds (inclusive on left/top, exclusive on right/bottom)
pub fn between(left: f32, right: f32, top: f32, bottom: f32, loc : Vec2) -> bool {
    let horizontal = loc.x >= left && loc.x < right;
    let vertical = loc.y >= top && loc.y < bottom;
    horizontal && vertical
}

/// A 2D vector type for representing positions, velocities, and dimensions.
/// 
/// Supports common vector operations including addition, subtraction,
/// scalar multiplication/division, dot product, and normalization.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Vec2
{
    /// X component of the vector
    pub x: f32,
    /// Y component of the vector
    pub y: f32,
}

impl Vec2 {
    /// Creates a new Vec2 with the specified components.
    /// 
    /// # Arguments
    /// * `x` - X component value
    /// * `y` - Y component value
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    /// Calculates the squared length (magnitude) of the vector.
    /// 
    /// # Returns
    /// The squared length (x² + y²), useful for avoiding expensive square root operations.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Calculates the length (magnitude) of the vector.
    /// 
    /// # Returns
    /// The Euclidean length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a normalized (unit length) version of this vector.
    /// 
    /// # Returns
    /// A vector with the same direction but length of 1, or the original vector if length is 0.
    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    /// Calculates the dot product with another vector.
    /// 
    /// # Arguments
    /// * `other` - The other vector to compute dot product with
    /// 
    /// # Returns
    /// The scalar dot product value.
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the distance to another point.
    /// 
    /// # Arguments
    /// * `other` - The other point to measure distance to
    /// 
    /// # Returns
    /// The Euclidean distance between this point and the other.
    pub fn distance(&self, other: &Vec2) -> f32 {
        (*self - *other).length()
    }
}

impl Default for Vec2 {
    /// Creates a zero vector (0, 0).
    fn default() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    /// Adds two vectors component-wise.
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    /// Subtracts two vectors component-wise.
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    /// Multiplies vector by a scalar.
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    /// Divides vector by a scalar.
    fn div(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    /// Negates both components of the vector.
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vec2 {
    /// Adds another vector to this vector in-place.
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    /// Subtracts another vector from this vector in-place.
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    /// Multiplies this vector by a scalar in-place.
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vec2 {
    /// Divides this vector by a scalar in-place.
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
