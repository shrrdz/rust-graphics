use super::color::Color;
use super::vertex::*;
use crate::algebra::vector3::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Model
{
    pub vertices: Vec<Vertex>,
}

struct Face
{
    v1: usize, v2: usize, v3: usize,
    t1: usize, t2: usize, t3: usize,
    n1: usize, n2: usize, n3: usize,
}

impl Model
{
    pub fn load_obj(path: &str, uv_factor: f32) -> Self
    {
        let mut model = Self { vertices: Vec::new() };

        let file = File::open(path).expect("Error : failed to open file");
        let reader = BufReader::new(file);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for line in reader.lines()
        {
            if let Ok(line) = line
            {
                let mut parts = line.split_whitespace();

                if let Some(token) = parts.next()
                {
                    match token
                    {
                        "v" =>
                        {
                            let x: f32 = parts.next().unwrap().parse().unwrap();
                            let y: f32 = parts.next().unwrap().parse().unwrap();
                            let z: f32 = parts.next().unwrap().parse().unwrap();

                            positions.push(Vector3::create(x, y, z));
                        }

                        "vn" =>
                        {
                            let x: f32 = parts.next().unwrap().parse().unwrap();
                            let y: f32 = parts.next().unwrap().parse().unwrap();
                            let z: f32 = parts.next().unwrap().parse().unwrap();

                            normals.push(Vector3::create(x, y, z));
                        }

                        "vt" =>
                        {
                            let x: f32 = parts.next().unwrap().parse().unwrap();
                            let y: f32 = parts.next().unwrap().parse().unwrap();

                            uvs.push(Vector3::create(x, y, 0.0));
                        }

                        "f" =>
                        {
                            let mut face = Face
                            {
                                v1: 0, t1: 0, n1: 0,
                                v2: 0, t2: 0, n2: 0,
                                v3: 0, t3: 0, n3: 0,
                            };

                            let mut indices = Vec::new();

                            for part in parts
                            {
                                let mut subparts = part.split('/');

                                let v: usize = subparts.next().unwrap().parse().unwrap();
                                let t: usize = subparts.next().unwrap().parse().unwrap();
                                let n: usize = subparts.next().unwrap().parse().unwrap();
                                
                                indices.push((v, t, n));
                            }

                            face.v1 = indices[0].0;
                            face.t1 = indices[0].1;
                            face.n1 = indices[0].2;

                            face.v2 = indices[1].0;
                            face.t2 = indices[1].1;
                            face.n2 = indices[1].2;
                            
                            face.v3 = indices[2].0;
                            face.t3 = indices[2].1;
                            face.n3 = indices[2].2;

                            model.process(face.v1, face.t1, face.n1, &positions, &normals, &uvs, uv_factor);
                            model.process(face.v2, face.t2, face.n2, &positions, &normals, &uvs, uv_factor);
                            model.process(face.v3, face.t3, face.n3, &positions, &normals, &uvs, uv_factor);
                        }

                        _ => { }
                    }
                }
            }
        }

        model
    }

    fn process(&mut self, v: usize, t: usize, n: usize, positions: &Vec<Vector3>, normals: &Vec<Vector3>, uvs: &Vec<Vector3>, uv_factor: f32)
    {
        let mut vertex: Vertex = Vertex::partial
        (
            -positions[v -1].x, positions[v -1].y, positions[v -1].z, // note the x coordinate
            uvs[t - 1].x, uvs[t - 1].y, 
            normals[n - 1]
        );
        
        vertex.color = Color::create(0.5, 0.5, 0.5);

        vertex.u *= uv_factor;
        vertex.v *= uv_factor;

        self.vertices.push(vertex);
    }
}