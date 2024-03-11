use super::color::*;

#[derive(Clone, Copy)]
pub struct Vertex
{
    pub x: f32,
    pub y: f32,

    pub color: Color,
}

impl Vertex
{
    pub fn blank() -> Self
    {
        Self
        {
            x: 0.0, y: 0.0,
            
            color: Color::create(0.0, 0.0, 0.0),
        }
    }

    pub fn create(x: f32, y: f32, color: Color) -> Self
    {
        Self { x, y, color }
    }
}