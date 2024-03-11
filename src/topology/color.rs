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

impl std::ops::Add<Color> for Color
{
    type Output = Color;

    fn add(self, scalar: Color) -> Color
    {
        Color
        {
            r: self.r + scalar.r, 
            g: self.g + scalar.g,
            b: self.b + scalar.b,
        }
    }
}

impl std::ops::Mul<f32> for Color
{
    type Output = Color;

    fn mul(self, scalar: f32) -> Color
    {
        Color
        {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl std::ops::Mul<Color> for Color
{
    type Output = Color;

    fn mul(self, scalar: Color) -> Color
    {
        Color
        {
            r: self.r * scalar.r,
            g: self.g * scalar.g,
            b: self.b * scalar.b,
        }
    }
}