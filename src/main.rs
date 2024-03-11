mod video;
mod topology;

use topology::{color::*, vertex::*};
use video::{render::Render, screen::*};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main()
{
    let screen = Screen::create(WIDTH, HEIGHT);
    
    let mut render = Render::create(screen);

    loop
    {
        render.screen.input();
        render.screen.tick();
        
        render.clear(sdl2::pixels::Color::RGB(24, 24, 24));
        
        render.triangle
        (
            &Vertex::create(100.0, 400.0, Color::create(1.0, 0.0, 0.0)),
            &Vertex::create(400.0, 100.0, Color::create(0.0, 1.0, 0.0)),
            &Vertex::create(700.0, 500.0, Color::create(0.0, 0.0, 1.0))
        );

        render.update();
    }
}