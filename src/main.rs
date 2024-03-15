mod video;
mod algebra;
mod topology;

use {algebra::vector3::*, topology::mesh::*, video::{render::*, screen::*, view::*}};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main()
{
    let screen = Screen::create(WIDTH, HEIGHT);
    let view = View::create(&Vector3::create(0.0, 0.0, -2.0), &Vector3::create(0.0, 0.0, 0.0));

    let mut render = Render::create(screen, view);

    let mut cube: Mesh = Mesh::cube();

    loop
    {
        render.screen.input(&mut render.view);
        render.screen.tick();
        
        render.clear(sdl2::pixels::Color::RGB(24, 24, 24));

        render.process(&mut cube);
        
        render.update();
    }
}