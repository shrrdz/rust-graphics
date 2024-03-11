use super::vector3::*;

#[derive(Clone, Copy)]
pub struct Matrix4x4
{
    // matrix data are in row-major order
    data: [[f32; 4]; 4],
}

impl Matrix4x4
{
    pub fn identity() -> Self
    {
        let mut data = [[0.0; 4]; 4];

        for i in 0..4
        {
            data[i][i] = 1.0;
        }

        Self { data }
    }

    pub fn translate(x: f32, y:f32, z: f32) -> Self
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();

        result.data[0][3] = x;
        result.data[1][3] = y;
        result.data[2][3] = z;

        result
    }

    pub fn scale(x: f32, y:f32, z: f32) -> Self
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();

        result.data[0][0] = x;
        result.data[1][1] = y;
        result.data[2][2] = z;

        result
    }

    pub fn rotate_y(angle: f32) -> Self
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();

        result.data[0][0] = f32::cos(f32::to_radians(angle));
        result.data[0][2] = f32::sin(f32::to_radians(angle));
        result.data[2][0] = -f32::sin(f32::to_radians(angle));
        result.data[2][2] = f32::cos(f32::to_radians(angle));

        result
    }

    pub fn view(eye: &Vector3, at: &Vector3, up: &Vector3) -> Self
    {
        let mut view: Matrix4x4 = Matrix4x4::identity();

        let z: Vector3 = (*at - *eye).normalized();                   // forward
        let x: Vector3 = Vector3::cross(up, &z).normalized();  // right
        let y: Vector3 = Vector3::cross(&z, &x);               // up

        view.data[0][0] = x.x;
        view.data[1][0] = y.x;
        view.data[2][0] = z.x;

        view.data[0][1] = x.y;
        view.data[1][1] = y.y;
        view.data[2][1] = z.y;

        view.data[0][2] = x.z;
        view.data[1][2] = y.z;
        view.data[2][2] = z.z;

        view.data[0][3] = -Vector3::dot(&x, eye);
        view.data[1][3] = -Vector3::dot(&y, eye);
        view.data[2][3] = -Vector3::dot(&z, eye);

        view
    }

    // projection matrix - perspective
    pub fn perspective(fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Self
    {
        let mut projection: Matrix4x4 = Matrix4x4::identity();

        let tan_half_fov: f32 = f32::tan(f32::to_radians(fov_degrees) / 2.0);

        projection.data[0][0] = 1.0 / (aspect_ratio * tan_half_fov);
        projection.data[1][1] = 1.0 / tan_half_fov;
        projection.data[2][2] = far / (far - near);
        projection.data[2][3] = (-far * near) / (far - near);
        projection.data[3][2] = 1.0;
        projection.data[3][3] = 0.0;

        projection
    }

    pub fn get(&self, row: usize, column: usize) -> f32
    {
        self.data[row][column]
    }
}

impl std::ops::Mul<Matrix4x4> for Matrix4x4
{
    type Output = Matrix4x4;

    fn mul(self, scalar: Matrix4x4) -> Matrix4x4
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();

        for i in 0..4
        {
            for j in 0..4
            {
                result.data[i][j] =

                self.data[i][0] * scalar.data[0][j] +
                self.data[i][1] * scalar.data[1][j] +
                self.data[i][2] * scalar.data[2][j] +
                self.data[i][3] * scalar.data[3][j];
            }
        }

        Matrix4x4 { data: result.data }
    }
}