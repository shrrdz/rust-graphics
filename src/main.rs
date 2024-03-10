mod video;

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

        render.update();
    }
}