use macroquad::color::{Color, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use crate::functionals::rect::Rect;
use crate::functionals::vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Button
{
    button : Rect,
    text: &'static str,
    color: Color
}

impl Button {
    pub fn new(x: f32, y: f32, w: f32, h: f32, text: &'static str, color: Color) -> Button {
        Button{
            button : Rect::new(x, y, w, h),
            text,
            color
        }
    }

    pub fn clicked(&self) -> bool {
        if macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            let mouse_pos = macroquad::input::mouse_position();
            if self.button.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                return true;
            }
        }
        false
    }
    
    pub fn draw(&self)
    {
        draw_rectangle(self.button.x, self.button.y, self.button.w, self.button.h, self.color);
        draw_text(&*self.text, self.button.x, self.button.y, 10.0, WHITE);
    }
}