use crate::{algebra::{matrix4x4::*, vector3::*}, WIDTH, HEIGHT};

pub struct View
{
    pub position: Vector3,
    pub rotation: Vector3,

    forward: Vector3,
    right: Vector3,
    up: Vector3,
}

impl View
{
    pub fn create(position: &Vector3, rotation: &Vector3) -> Self
    {
        let mut view = Self
        {
            position: *position,
            rotation: *rotation,

            forward: Vector3::create(0.0, 0.0, 1.0),
            right:   Vector3::zero(),
            up:      Vector3::create(0.0, 1.0, 0.0),
        };

        view.orientate();

        view
    }

    pub fn view(&self) -> Matrix4x4
    {
        Matrix4x4::view(&self.position, &(self.position + self.forward), &self.up)
    }

    pub fn perspective(&self) -> Matrix4x4
    {
        Matrix4x4::perspective(75.0, WIDTH as f32 / HEIGHT as f32, 0.1, 1000.0)
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32)
    {
        // calculate the motion vector in the view's local coordinates
        let motion: Vector3 = (self.forward * z) + (self.right * x) + (self.up * y);

        self.position = self.position + motion;
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32)
    {
        self.rotation.x += pitch;
        self.rotation.y += yaw;

        // constrain the pitch
        if self.rotation.x >= 89.0 { self.rotation.x = 89.0 };
        if self.rotation.x <= -89.0 { self.rotation.x = -89.0 };

        self.orientate();
    }
    
    // updates the view's forward, right and up vector
    pub fn orientate(&mut self)
    {
        let front: Vector3 = Vector3::create
        (
            f32::sin(f32::to_radians(self.rotation.y)) * f32::cos(f32::to_radians(self.rotation.x)),
            f32::sin(f32::to_radians(self.rotation.x)),
            f32::cos(f32::to_radians(self.rotation.y)) * f32::cos(f32::to_radians(self.rotation.x))
        );
        
        let world_up: Vector3 = Vector3::create(0.0, 1.0, 0.0);

        self.forward = front.normalized();
        self.right   = Vector3::cross(&world_up, &self.forward).normalized();
        self.up      = Vector3::cross(&self.forward, &self.right).normalized();
    }
}