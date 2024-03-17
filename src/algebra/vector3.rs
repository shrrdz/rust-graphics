#[derive(Clone, Copy)]
pub struct Vector3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3
{
    pub fn create(x: f32, y: f32, z: f32) -> Self
    {
        Self { x, y, z }
    }

    pub fn zero() -> Self
    {
        Self { x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn length(&self) -> f32
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn opposite(&self) -> Self
    {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn normalized(&self) -> Self
    {
        Self { x: self.x / self.length(), y: self.y / self.length(), z: self.z / self.length() }
    }

    pub fn dot(v1: &Vector3, v2: &Vector3) -> f32
    {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Vector3, v2: &Vector3) -> Self
    {
        Self { x: v1.y * v2.z - v1.z * v2.y, y: v1.z * v2.x - v1.x * v2.z, z: v1.x * v2.y - v1.y * v2.x }
    }

    pub fn reflect(incident: &Vector3, normal: &Vector3) -> Self
    {
        let twodot: f32 = 2.0 * Vector3::dot(&incident, &normal);
        
        let result: Vector3 = (*incident - *normal * twodot).normalized();

        result
    }
}

impl std::ops::Add for Vector3
{
    type Output = Vector3;

    fn add(self, scalar: Vector3) -> Vector3
    {
        Vector3
        {
            x: self.x + scalar.x,
            y: self.y + scalar.y,
            z: self.z + scalar.z,
        }
    }
}

impl std::ops::Sub<Vector3> for Vector3
{
    type Output = Vector3;

    fn sub(self, scalar: Vector3) -> Vector3
    {
        Vector3
        {
            x: self.x - scalar.x,
            y: self.y - scalar.y,
            z: self.z - scalar.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3
{
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3
    {
        Vector3
        {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<Vector3> for Vector3
{
    type Output = Vector3;

    fn mul(self, scalar: Vector3) -> Vector3
    {
        Vector3
        {
            x: self.x * scalar.x,
            y: self.y * scalar.y,
            z: self.z * scalar.z,
        }
    }
}

impl std::ops::Div<f32> for Vector3
{
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3
    {
        Vector3
        {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}