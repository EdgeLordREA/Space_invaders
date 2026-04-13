use crate::functionals::vec2::Vec2;

/// Mathematical utilities and vector operations.
/// 
/// This module provides mathematical helper functions
/// for game calculations including collision detection, movement, and physics.

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

