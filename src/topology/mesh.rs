use super::{color::*, part::*, vertex::*};
use crate::algebra::{matrix4x4::*, vector3::*};

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
            Vertex::create(-0.5, 0.5, -0.5, Color::create(0.8, 0.8, 0.8), 0.0, 0.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(0.8, 0.8, 0.8), 1.0, 0.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(0.8, 0.8, 0.8), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(0.8, 0.8, 0.8),0.0, 1.0),

            // bottom
            Vertex::create(-0.5, -0.5, 0.5, Color::create(0.2, 0.2, 0.2), 0.0, 0.0),
            Vertex::create(0.5, -0.5, 0.5, Color::create(0.2, 0.2, 0.2), 1.0, 0.0),
            Vertex::create(0.5, -0.5, -0.5, Color::create(0.2, 0.2, 0.2), 1.0, 1.0),
            Vertex::create(-0.5, -0.5, -0.5, Color::create(0.2, 0.2, 0.2), 0.0, 1.0),

            // front
            Vertex::create(-0.5, -0.5, -0.5, Color::create(0.8, 0.2, 0.2), 0.0, 0.0),
            Vertex::create(0.5, -0.5, -0.5, Color::create(0.8, 0.2, 0.2), 1.0, 0.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(0.8, 0.2, 0.2), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, -0.5, Color::create(0.8, 0.2, 0.2), 0.0, 1.0),

            // back
            Vertex::create(0.5, -0.5, 0.5, Color::create(0.2, 0.8, 0.2), 0.0, 0.0),
            Vertex::create(-0.5, -0.5, 0.5, Color::create(0.2, 0.8, 0.2), 1.0, 0.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(0.2, 0.8, 0.2), 1.0, 1.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(0.2, 0.8, 0.2), 0.0, 1.0),

            // left
            Vertex::create(-0.5, -0.5, 0.5, Color::create(0.8, 0.8, 0.2), 0.0, 0.0),
            Vertex::create(-0.5, -0.5, -0.5, Color::create(0.8, 0.8, 0.2), 1.0, 0.0),
            Vertex::create(-0.5, 0.5, -0.5, Color::create(0.8, 0.8, 0.2), 1.0, 1.0),
            Vertex::create(-0.5, 0.5, 0.5, Color::create(0.8, 0.8, 0.2), 0.0, 1.0),

            // right
            Vertex::create(0.5, -0.5, -0.5, Color::create(0.2, 0.2, 0.8), 0.0, 0.0),
            Vertex::create(0.5, -0.5, 0.5, Color::create(0.2, 0.2, 0.8), 1.0, 0.0),
            Vertex::create(0.5, 0.5, 0.5, Color::create(0.2, 0.2, 0.8), 1.0, 1.0),
            Vertex::create(0.5, 0.5, -0.5, Color::create(0.2, 0.2, 0.8), 0.0, 1.0),
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

        cube
    }
}