use super::{screen::*, view::*, light::*};
use crate::{algebra::matrix4x4::*, topology::{mesh::*, part::*, color::*, vertex::*}};

use sdl2::rect::Point;

pub struct Render
{
    pub screen: Screen,
    pub view: View,
}

impl Render
{
    pub fn create(screen: Screen, view: View) -> Self
    {
        Self { screen, view }
    }

    pub fn update(&mut self)
    {
        self.screen.canvas.present();
    }
    
    pub fn clear(&mut self, color: sdl2::pixels::Color)
    {
        for i in self.screen.depth_buffer.iter_mut()
        {
            *i = 1.0;
        }
        
        self.screen.canvas.set_draw_color(color);
        self.screen.canvas.clear();
    }

    pub fn pixel(&mut self, x: i32, y: i32, z: f32, color: Color)
    {
        if x >= 0 && x < self.screen.width && y >= 0 && y < self.screen.height
        {
            // perform a depth test
            if z < self.screen.depth_buffer[(self.screen.width * y + x) as usize]
            {
                self.screen.depth_buffer[(self.screen.width * y + x) as usize] = z;
                self.screen.canvas.set_draw_color(sdl2::pixels::Color::RGB((color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8));
                self.screen.canvas.draw_point(Point::new(x, y)).unwrap();
            }
        }
    }

    pub fn triangle(&mut self, a: &Vertex, b: &Vertex, c: &Vertex)
    {
        // signed area of the triangle
        let area: f32 = Vertex::signed_triangle_area(a, b, c);

        // perform backface culling
        if area > 0.0 { return; }

        // bounding box of the triangle
        let xmin: i32 = f32::min(f32::min(a.x, b.x), c.x).floor() as i32;
        let xmax: i32 = f32::max(f32::max(a.x, b.x), c.x).floor() as i32;
        let ymin: i32 = f32::min(f32::min(a.y, b.y), c.y).ceil() as i32;
        let ymax: i32 = f32::max(f32::max(a.y, b.y), c.y).ceil() as i32;

        let reciprocal_area = 1.0 / area;

        for y in ymin ..= ymax
        {
            for x in xmin ..= xmax
            {
                // barycentric coordinates
                let alpha: f32 = ((b.y - c.y) * (x as f32 - c.x) + (c.x - b.x) * (y as f32 - c.y)) * reciprocal_area;
                let beta: f32 = ((c.y - a.y) * (x as f32 - c.x) + (a.x - c.x) * (y as f32 - c.y)) * reciprocal_area;
                let gamma: f32 = 1.0 - alpha - beta;

                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
                {
                    let mut frag: Vertex = Vertex::blank();

                    // depth interpolation
                    frag.z = a.z * alpha + b.z * beta + c.z * gamma;
                    // color interpolation
                    frag.color = a.color * alpha + b.color * beta + c.color * gamma;
                    // reciprocal interpolation
                    frag.one = a.one * alpha + b.one * beta + c.one * gamma;
                    // uv coordinates interpolation
                    frag.u = a.u * alpha + b.u * beta + c.u * gamma;
                    frag.v = a.v * alpha + b.v * beta + c.v * gamma;
                    // normal interpolation
                    frag.normal = a.normal * alpha + b.normal * beta + c.normal * gamma;

                    // perspective-correct interpolation
                    frag.color = frag.color / frag.one;
                    frag.u = frag.u / frag.one;
                    frag.v = frag.v / frag.one;
                    
                    self.pixel(x, y, frag.z, phong(&mut frag, &self.view));
                }
            }
        }
    }

    pub fn process(&mut self, mesh: &mut Mesh)
    {
        mesh.update();
        
        let transformation_matrix: Matrix4x4 = self.view.perspective() * self.view.view() * mesh.model;

        let mut vertices: Vec<Vertex> = mesh.vertices.clone();

        // transform the vertices & their normals in 3D space
        for vertex in &mut vertices
        {
            *vertex = vertex.transform(&transformation_matrix).image_space().screen_space();
            vertex.normal = vertex.transform_normal(&mesh.model);
        }

        for part in &mesh.parts
        {
            let mut start: usize = part.index;
            
            match part.topology
            {
                Topology::TRIANGLE =>
                {
                    for _ in 0 .. part.count
                    {
                        let mut a = mesh.vertices[mesh.indices[start]].transform(&transformation_matrix);
                        let mut b = mesh.vertices[mesh.indices[start + 1]].transform(&transformation_matrix);
                        let mut c = mesh.vertices[mesh.indices[start + 2]].transform(&transformation_matrix);

                        // ensure that only the triangles that are within the view space are drawn
                        if !Vertex::out_of_view(&a, &b, &c)
                        {
                            a = vertices[mesh.indices[start]];
                            b = vertices[mesh.indices[start + 1]];
                            c = vertices[mesh.indices[start + 2]];

                            // all vertices are now ready to be rendered
                            self.triangle(&a, &b, &c);
                        }

                        start += 3;
                    }
                }
            }
        }
    }
}