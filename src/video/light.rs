use super::view::*;
use crate::{algebra::vector3::*, topology::{color::*, vertex::*}};

// Phong reflection
pub fn phong(vt: &mut Vertex, view: &View) -> Color
{
    let normal: Vector3 = vt.normal.normalized();

    let light_direction: Vector3 = Vector3::create(-1.0, -1.0, 1.0).opposite().normalized();

    // Lambert's cosine law
    let lambertian: f32 = f32::max(0.0, Vector3::dot(&light_direction, &normal));

    // intensity of the specular highlight
    let specular_exponent: f32 = 48.0;

    let mut specular_highlight: f32 = 0.0;

    // if there is no diffuse lighting hitting the surface, don't bother calculating the specular
    if lambertian != 0.0
    {
        let view_direction: Vector3 = (view.position - Vector3::create(vt.x, vt.y, vt.z)).normalized();
        let reflect_direction: Vector3 = Vector3::reflect(light_direction.opposite(), normal);

        specular_highlight = f32::powf(f32::max(0.0, Vector3::dot(&view_direction, &reflect_direction)), specular_exponent);
    }

    let ambient: Color = vt.color * Color::create(0.1, 0.1, 0.1);
    let diffuse: Color = vt.color * lambertian;
    let specular: Color = Color::create(1.0, 1.0, 1.0) * specular_highlight;

    ambient + diffuse + specular
}