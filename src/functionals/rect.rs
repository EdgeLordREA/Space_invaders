use serde::Deserialize;
// Assuming Vec2 is in your crate
use crate::functionals::vec2::Vec2;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Creates a rectangle from a top-left point and a size vector
    pub fn from_vecs(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }

    pub fn center(self) -> Vec2 {
        Vec2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    // --- Helpers for cleaner collision math ---

    pub fn left(self) -> f32 { self.x }
    pub fn right(self) -> f32 { self.x + self.w }
    pub fn top(self) -> f32 { self.y }
    pub fn bottom(self) -> f32 { self.y + self.h }

    // --- Point Queries ---

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.left() &&
            point.x <= self.right() &&
            point.y >= self.top() &&
            point.y <= self.bottom()
    }

    pub fn intersects(self, other: Rect) -> bool {
        self.left() < other.right() &&
            self.right() > other.left() &&
            self.top() < other.bottom() &&
            self.bottom() > other.top()
    }

    /// Returns a new Rect expanded by a padding value on all sides
    pub fn inflate(self, amount: f32) -> Self {
        Self {
            x: self.x - amount,
            y: self.y - amount,
            w: self.w + amount * 2.0,
            h: self.h + amount * 2.0,
        }
    }

    pub fn get_closest_point(self, point: Vec2) -> Vec2 {
        let closest_x = point.x.clamp(self.x, self.x + self.w);
        let closest_y = point.y.clamp(self.y, self.y + self.h);
        Vec2::new(closest_x, closest_y)
    }
}