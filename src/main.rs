mod video;
mod algebra;
mod topology;

use {algebra::vector3::*, topology::{mesh::*, model::*}, video::{render::*, screen::*, view::*}};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main()
{
    let screen = Screen::create(WIDTH, HEIGHT);
    let view = View::create(&Vector3::create(0.0, 0.0, -3.0), &Vector3::create(0.0, 0.0, 0.0));

    let mut render = Render::create(screen, view);

//  let mut mesh: Mesh = Mesh::sphere(40, 20, 1.0);

    let model_data = Model::load_obj("assets/bunny.obj", 1.0);
    let mut model: Mesh = Mesh::converted(&model_data);

    loop
    {
        render.screen.input(&mut render.view);
        render.screen.tick();
        
        render.clear(sdl2::pixels::Color::RGB(24, 24, 24));

        model.rotation.y += 30.0 * render.screen.delta_time;

        render.process(&mut model);
        
        render.update();
    }
}