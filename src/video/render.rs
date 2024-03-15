use super::screen::*;
use crate::topology::{color::*, vertex::*};

use sdl2::rect::Point;

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
    
    pub fn clear(&mut self, color: sdl2::pixels::Color)
    {
        for i in self.screen.depth_buffer.iter_mut()
        {
            *i = 1.0;
        }
        
        self.screen.canvas.set_draw_color(color);
        self.screen.canvas.clear();
    }

    pub fn pixel(&mut self, x: i32, y: i32, z: f32, color: Color)
    {
        if x >= 0 && x < self.screen.width && y >= 0 && y < self.screen.height
        {
            // perform a depth test
            if z < self.screen.depth_buffer[(self.screen.width * y + x) as usize]
            {
                self.screen.depth_buffer[(self.screen.width * y + x) as usize] = z;
                self.screen.canvas.set_draw_color(sdl2::pixels::Color::RGB((color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8));
                self.screen.canvas.draw_point(Point::new(x, y)).unwrap();
            }
        }
    }

    pub fn triangle(&mut self, a: &Vertex, b: &Vertex, c: &Vertex)
    {
        // signed area of the triangle
        let area: f32 = Vertex::signed_triangle_area(a, b, c);

        // bounding box of the triangle
        let xmin: i32 = f32::min(f32::min(a.x, b.x), c.x).floor() as i32;
        let xmax: i32 = f32::max(f32::max(a.x, b.x), c.x).floor() as i32;
        let ymin: i32 = f32::min(f32::min(a.y, b.y), c.y).ceil() as i32;
        let ymax: i32 = f32::max(f32::max(a.y, b.y), c.y).ceil() as i32;

        let reciprocal_area = 1.0 / area;

        for y in ymin ..= ymax
        {
            for x in xmin ..= xmax
            {
                // barycentric coordinates
                let alpha: f32 = ((b.y - c.y) * (x as f32 - c.x) + (c.x - b.x) * (y as f32 - c.y)) * reciprocal_area;
                let beta: f32 = ((c.y - a.y) * (x as f32 - c.x) + (a.x - c.x) * (y as f32 - c.y)) * reciprocal_area;
                let gamma: f32 = 1.0 - alpha - beta;

                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
                {
                    let mut frag: Vertex = Vertex::blank();

                    // depth interpolation
                    frag.z = a.z * alpha + b.z * beta + c.z * gamma;
                    // color interpolation
                    frag.color = a.color * alpha + b.color * beta + c.color * gamma;
                    // reciprocal interpolation
                    frag.one = a.one * alpha + b.one * beta + c.one * gamma;
                    
                    // perspective-correct interpolation
                    frag.color = frag.color / frag.one;

                    self.pixel(x, y, frag.z, frag.color);
                }
            }
        }
    }
}