use super::color::*;
use crate::{algebra::matrix4x4::*, WIDTH, HEIGHT};

#[derive(Clone, Copy)]
pub struct Vertex
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,

    pub u: f32,
    pub v: f32,

    pub one: f32,

    pub color: Color,
}

impl Vertex
{
    pub fn blank() -> Self
    {
        Self
        {
            x: 0.0, y: 0.0, z: 0.0, w: 1.0,
            
            color: Color::create(0.0, 0.0, 0.0),

            u: 0.0, v: 0.0, one: 1.0,
        }
    }

    pub fn create(x: f32, y: f32, z: f32, color: Color, u: f32, v: f32) -> Self
    {
        Self { x, y, z, w: 1.0, color, u, v, one: 1.0 }
    }

    pub fn partial(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self
    {
        Self { x, y, z, w: 1.0, color: Color::blank(), u, v, one: 1.0 }
    }

    pub fn full(x: f32, y: f32, z: f32, color: Color, u: f32, v: f32) -> Self
    {
        Self { x, y, z, w: 1.0, color, u, v, one: 1.0 }
    }

    // transforms the vertex into image space (NDC) using perspective division
    //
    //      [-1, 1]-----[1, 1]
    //      |                |
    //      |     [0, 0]     |
    //      |                |
    //      [-1,-1]-----[1,-1]
    //
    pub fn image_space(&self) -> Self
    {
        Self
        { 
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
            w: 1.0,
            
            color: self.color / self.w,

            u: self.u / self.w,
            v: self.v / self.w,

            one: self.one / self.w,
        }
    }

    // transform the vertex into screen space
    pub fn screen_space(&self) -> Self
    {
        Self
        {
            x: ((self.x + 1.0) * WIDTH as f32) / 2.0,
            y: ((1.0 - self.y) * HEIGHT as f32) / 2.0,
            z: self.z,
            w: self.w,
            
            color: self.color,

            u: self.u,
            v: self.v,

            one: self.one,
        }
    }

    pub fn transform(&self, matrix: &Matrix4x4) -> Self
    {
        Self
        {
            x: matrix.get(0, 0) * self.x + matrix.get(0, 1) * self.y + matrix.get(0, 2) * self.z + matrix.get(0, 3) * self.w,
            y: matrix.get(1, 0) * self.x + matrix.get(1, 1) * self.y + matrix.get(1, 2) * self.z + matrix.get(1, 3) * self.w,
            z: matrix.get(2, 0) * self.x + matrix.get(2, 1) * self.y + matrix.get(2, 2) * self.z + matrix.get(2, 3) * self.w,
            w: matrix.get(3, 0) * self.x + matrix.get(3, 1) * self.y + matrix.get(3, 2) * self.z + matrix.get(3, 3) * self.w,

            color: self.color, u: self.u, v: self.v, one: self.one,
        }
    }

    // returns a scalar equal to the signed area of the given triangle (used for backface culling)
    pub fn signed_triangle_area(a: &Vertex, b: &Vertex, c: &Vertex) -> f32
    {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
}