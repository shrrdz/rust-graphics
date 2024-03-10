use sdl2::{video::Window, render::Canvas};

pub struct Screen
{
    pub width: i32,
    pub height: i32,

    pub sdl: sdl2::Sdl,
    pub canvas: Canvas<Window>,
}

impl Screen
{
    pub fn create(width: i32, height: i32) -> Self
    {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("gl_rust", width as u32, height as u32).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Self
        {
            width,
            height,
            
            sdl,
            canvas,
        }
    }
}