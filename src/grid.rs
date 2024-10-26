use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::color::Color;

pub struct Grid {
    pub size : i32,
    pub color : Color
}

impl Grid {
    pub fn draw(self : &Grid, width : i32, height : i32, r : &mut RaylibDrawHandle) {

        for v in 0..width {
            r.draw_line(v * self.size, 0, v * self.size, height, self.color);
        }

        for h in 0..height {
            r.draw_line(0, h * self.size, width, h * self.size, self.color);
        }
    }
}