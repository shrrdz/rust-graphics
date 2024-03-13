mod video;
mod algebra;
mod topology;

use algebra::{vector3::*, matrix4x4::*};
use topology::{color::*, vertex::*};
use video::{render::*, screen::*, view::*};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main()
{
    let screen = Screen::create(WIDTH, HEIGHT);
    let view = View::create(&Vector3::create(0.0, 0.5, -2.0), &Vector3::create(0.0, 0.0, 0.0));

    let mut render = Render::create(screen);

    loop
    {
        render.screen.input();
        render.screen.tick();
        
        render.clear(sdl2::pixels::Color::RGB(24, 24, 24));
        
        let transformation_matrix: Matrix4x4 = view.perspective() * view.view() * Matrix4x4::identity();

        render.triangle
        (
            &Vertex::create(-1.0, 0.0, 0.0, Color::create(1.0, 0.0, 0.0)).transform(&transformation_matrix).image_space().screen_space(),
            &Vertex::create(1.0, 0.0, 0.0, Color::create(1.0, 1.0, 0.0)).transform(&transformation_matrix).image_space().screen_space(),
            &Vertex::create(0.0, 1.0, 0.0, Color::create(0.0, 0.0, 1.0)).transform(&transformation_matrix).image_space().screen_space()
        );

        render.update();
    }
}