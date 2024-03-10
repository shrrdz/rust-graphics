use super::screen::*;

use sdl2::rect::Point;
use sdl2::pixels::Color;

pub struct Render
{
    pub screen: Screen,
}

impl Render
{
    pub fn create(screen: Screen) -> Self
    {
        Self { screen }
    }

    pub fn update(&mut self)
    {
        self.screen.canvas.present();
    }
    
    pub fn clear(&mut self, color: Color)
    {
        self.screen.canvas.set_draw_color(color);
        self.screen.canvas.clear();
    }

    pub fn pixel(&mut self, x: i32, y: i32, color: Color)
    {
        if x >= 0 && x < self.screen.width && y >= 0 && y < self.screen.height
        {
            self.screen.canvas.set_draw_color(color);
            self.screen.canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }
}