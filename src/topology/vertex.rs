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

    // returns a scalar equal to the signed area of the given triangle (used for backface culling)
    pub fn signed_triangle_area(a: &Vertex, b: &Vertex, c: &Vertex) -> f32
    {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
}