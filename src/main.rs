mod video;

use video::screen::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main()
{
    let mut screen = Screen::create(WIDTH, HEIGHT);

    loop
    {
        screen.input();
        screen.tick();
        
        screen.canvas.set_draw_color(sdl2::pixels::Color::RGB(24, 24, 24));
        screen.canvas.clear();

        screen.canvas.present();
    }
}