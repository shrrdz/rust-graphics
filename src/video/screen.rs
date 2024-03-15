use super::view::*;

use std::time::{Instant, Duration};

use sdl2::{video::Window, render::Canvas, event::Event, keyboard::{Keycode, Scancode}};

const SPEED: f32 = 2.0;

const FPS: u32 = 60;
const TARGET_TICK: u32 = 1000 / FPS;

pub struct Screen
{
    pub width: i32,
    pub height: i32,

    pub sdl: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    
    pub depth_buffer: Vec<f32>,

    previous_tick: Instant,
    pub delta_time: f32,
}

impl Screen
{
    pub fn create(width: i32, height: i32) -> Self
    {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("gl_rust", width as u32, height as u32).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();

        let mut depth_buffer = Vec::new();

        depth_buffer.resize((width * height) as usize, 1.0);
        
        Self
        {
            width,
            height,
            
            sdl,
            canvas,

            depth_buffer,

            previous_tick: Instant::now(),
            delta_time: 0.0,
        }
    }

    pub fn input(&self, view: &mut View)
    {
        let mut events = self.sdl.event_pump().unwrap();

        let key = events.keyboard_state();

        if key.is_scancode_pressed(Scancode::W) { view.translate(0.0, 0.0, SPEED * self.delta_time); }
        if key.is_scancode_pressed(Scancode::S) { view.translate(0.0, 0.0, -SPEED * self.delta_time); }
        if key.is_scancode_pressed(Scancode::A) { view.translate(-SPEED * self.delta_time, 0.0, 0.0); }
        if key.is_scancode_pressed(Scancode::D) { view.translate(SPEED * self.delta_time, 0.0, 0.0); }
        if key.is_scancode_pressed(Scancode::E) { view.translate(0.0, SPEED * self.delta_time, 0.0); }
        if key.is_scancode_pressed(Scancode::Q) { view.translate(0.0, -SPEED * self.delta_time, 0.0); }

        if key.is_scancode_pressed(Scancode::Left) { view.rotate(0.0, -30.0 * self.delta_time); }
        if key.is_scancode_pressed(Scancode::Right) { view.rotate(0.0, 30.0 * self.delta_time); }
        if key.is_scancode_pressed(Scancode::Up) { view.rotate(30.0 * self.delta_time, 0.0); }
        if key.is_scancode_pressed(Scancode::Down) { view.rotate(-30.0 * self.delta_time, 0.0); } 

        for event in events.poll_iter()
        {
            match event
            {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => std::process::exit(0),

                Event::Quit { .. } => std::process::exit(0),

                _ => { }
            }
        }      
    }

    pub fn tick(&mut self)
    {
        let now = Instant::now();
        let elapsed = now.duration_since(self.previous_tick);

        if elapsed.as_millis() < TARGET_TICK as u128
        {
            std::thread::sleep(Duration::from_millis((TARGET_TICK - elapsed.as_millis() as u32) as u64));
        }

        self.delta_time = elapsed.as_secs_f32();

        self.previous_tick = now;
    }
}