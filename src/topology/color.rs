#[derive(Clone, Copy)]
pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color
{
    pub fn blank() -> Self
    {
        Self { r: 0.0, g: 0.0, b: 0.0, }
    }

    pub fn create(r: f32, g: f32, b: f32) -> Self
    {
        Self { r, g, b, }
    }
}