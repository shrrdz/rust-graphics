use super::{color::*, part::*, model::*, vertex::*};
use crate::algebra::{matrix4x4::*, vector3::*};

use std::f32::consts::PI;

//--------------------------------------------------------------------------//
//                             == draw order ==
//    == order matters due to the way backface culling is implemented ==
//--------------------------------------------------------------------------//
//
//      [0, 1]---[1, 1]
//      |         /   |
//      |       /     |
//      |     /       |
//      |   /         |
//      [0, 0]---[1, 0]
//
//--------------------------------------------------------------------------//
//   [0, 0] -> [1, 0] -> [1, 1]       ::       [1, 1] -> [0, 1] -> [0, 0]
//--------------------------------------------------------------------------//

pub struct Mesh
{
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    pub parts: Vec<Part>,

    pub model: Matrix4x4,
}

impl Mesh
{
    pub fn update(&mut self)
    {
        self.model = 
        {
            Matrix4x4::translate(self.position.x, self.position.y, self.position.z) *
            Matrix4x4::rotate_y(self.rotation.y) *
            Matrix4x4::scale(self.scale.x, self.scale.y, self.scale.z)
        }
    }

    pub fn converted(model: &Model) -> Self
    {
        let mut mesh = Self
        { 
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::create(1.0, 1.0, 1.0),

            vertices: model.vertices.clone(),
            indices: Vec::new(), 
            parts: vec![Part::create(Topology::TRIANGLE, 0, model.vertices.len() / 3)],

            model: Matrix4x4::identity(),
        };

        for i in 0..model.vertices.len() / 3
        {
            mesh.indices.push(i * 3);
            mesh.indices.push(i * 3 + 1);
            mesh.indices.push(i * 3 + 2);
        }

        mesh
    }

    pub fn calculate_normals(&mut self, opposite: bool)
    {
        for i in (0 .. self.indices.len()).step_by(3)
        {
            let a: Vertex = self.vertices[self.indices[i]];
            let b: Vertex = self.vertices[self.indices[i + 1]];
            let c: Vertex = self.vertices[self.indices[i + 2]];

            let ab: Vector3 = Vector3::create(b.x - a.x, b.y - a.y, b.z - a.z);
            let ac: Vector3 = Vector3::create(c.x - a.x, c.y - a.y, c.z - a.z);

            let mut normal: Vector3 = Vector3::cross(&ab, &ac).normalized();

            if opposite { normal = normal.opposite(); }

            self.vertices[self.indices[i]].normal = normal;
            self.vertices[self.indices[i + 1]].normal = normal;
            self.vertices[self.indices[i + 2]].normal = normal;
        }
    }

    //--------------------------------------------------------------------------//
    //  primitives
    //--------------------------------------------------------------------------//
    pub fn triangle() -> Self
    {
        let mut triangle = Self
        { 
            position: Vector3::zero(), rotation: Vector3::zero(), scale: Vector3::create(1.0, 1.0, 1.0),
            vertices: Vec::new(), indices: Vec::new(), parts: Vec::new(),

            model: Matrix4x4::identity(),
        };
        
        triangle.vertices.push(Vertex::create(-0.5, -0.25, 0.0, Color::create(0.0, 0.0, 1.0), 0.0, 0.0));
        triangle.vertices.push(Vertex::create(0.5, -0.25, 0.0, Color::create(1.0, 1.0, 0.0), 1.0, 0.0));
        triangle.vertices.push(Vertex::create(0.0, 0.5, 0.0, Color::create(1.0, 0.0, 0.0), 0.5, 0.5));

        triangle.indices = vec![ 0, 1, 2];

        triangle.parts = vec![Part::create(Topology::TRIANGLE, 0, 1)];

        triangle
    }

    pub fn plane(color: &Color) -> Self
    {
        let mut plane = Self
        { 
            position: Vector3::zero(), rotation: Vector3::zero(), scale: Vector3::create(1.0, 1.0, 1.0),
            vertices: Vec::new(), indices: Vec::new(), parts: Vec::new(),

            model: Matrix4x4::identity(),
        };

        plane.vertices = vec!
        [
            Vertex::create(-0.5, 0.0, -0.5, *color, 0.0, 0.0),
            Vertex::create(0.5, 0.0, -0.5, *color, 1.0, 0.0),
            Vertex::create(0.5, 0.0, 0.5, *color, 1.0, 1.0),
            Vertex::create(-0.5, 0.0, 0.5, *color, 0.0, 1.0),
        ];

        plane.indices = vec!
        [
            0, 1, 2, 
            2, 3, 0,
        ];

        plane.parts = vec![Part::create(Topology::TRIANGLE, 0, 2)];

        plane.calculate_normals(true);

        plane
    }

    pub fn cube() -> Self
    {
        let mut cube = Self
        { 
            position: Vector3::zero(), rotation: Vector3::zero(), scale: Vector3::create(1.0, 1.0, 1.0),
            vertices: Vec::new(), indices: Vec::new(), parts: Vec::new(),

            model: Matrix4x4::identity(),
        };

        cube.vertices = vec!
        [
            // top
            Vertex::create(-0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 0.0, 0.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 1.0, 0.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0),0.0, 1.0),

            // bottom
            Vertex::create(-0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 0.0, 0.0),
            Vertex::create(0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 1.0, 0.0),
            Vertex::create(0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 1.0, 1.0),
            Vertex::create(-0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 0.0, 1.0),

            // front
            Vertex::create(-0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 0.0, 0.0),
            Vertex::create(0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 1.0, 0.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 0.0, 1.0),

            // back
            Vertex::create(0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 0.0, 0.0),
            Vertex::create(-0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 1.0, 0.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0), 1.0, 1.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0), 0.0, 1.0),

            // left
            Vertex::create(-0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 0.0, 0.0),
            Vertex::create(-0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 1.0, 0.0),
            Vertex::create(-0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0), 0.0, 1.0),

            // right
            Vertex::create(0.5, -0.5, -0.5, Color::create(1.0, 0.5, 0.0), 0.0, 0.0),
            Vertex::create(0.5, -0.5, 0.5, Color::create(1.0, 0.5, 0.0), 1.0, 0.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(1.0, 0.0, 0.0), 1.0, 1.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(1.0, 0.0, 0.0), 0.0, 1.0),
        ];

        cube.indices = vec!
        [ 
            0, 1, 2,
            0, 2, 3,

            4, 5, 6,
            4, 6, 7,

            8, 9, 10,
            8, 10, 11,

            12, 13, 14,
            12, 14, 15,

            16, 17, 18,
            16, 18, 19,

            20, 21, 22,
            20, 22, 23,
        ];

        cube.parts = vec![Part::create(Topology::TRIANGLE, 0, 12)];

        cube.calculate_normals(true);

        cube
    }

    pub fn sphere(segments: usize, rings: usize, radius: f32) -> Self
    {
        let mut sphere = Self
        { 
            position: Vector3::zero(), rotation: Vector3::zero(), scale: Vector3::create(1.0, 1.0, 1.0),
            vertices: Vec::new(), indices: Vec::new(), parts: Vec::new(),

            model: Matrix4x4::identity(),
        };

        for ring in 0 .. rings * 2
        {
            // polar angle in spherical coordinates
            let theta: f32 = ring as f32 / rings as f32 * PI;

            for segment in 0 ..= segments
            {
                // azimuthal angle in spherical coordinates
                let phi: f32 = segment as f32 / segments as f32 * 2.0 * PI;

                // vertices
                let x: f32 = radius * f32::sin(theta) * f32::cos(phi);
                let y: f32 = radius * f32::cos(theta);
                let z: f32 = radius * f32::sin(theta) * f32::sin(phi);

                sphere.vertices.push(Vertex::full(x, y, z, Color::create(0.2, 0.4, 0.8), segment as f32 / segments as f32, 1.0 - (ring as f32 / rings as f32), Vector3::create(x, y, z).normalized()));

                // indices
                let current: usize = ring * (segments + 1) + segment;
                let next: usize = current + segments + 1;

                sphere.indices.push(current);
                sphere.indices.push(next);
                sphere.indices.push(current + 1);

                sphere.indices.push(current + 1);
                sphere.indices.push(next);
                sphere.indices.push(next + 1);
            }
        }

        sphere.parts = vec![Part::create(Topology::TRIANGLE, 0, sphere.vertices.len())];

        sphere
    }
}