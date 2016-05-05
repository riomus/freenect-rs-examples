extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use im::{ImageBuffer, Rgba};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use std::sync::{Arc, Mutex};
use std::io::{Write, stderr};
use graphics::{clear, Image};

pub struct App {
    window: Window,
    texture: Texture
}


const OPENGL: OpenGL = OpenGL::V3_2;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;


impl App {
    pub fn new() -> App {
        let window =
        WindowSettings::new("Freenect-rs example", (WIDTH, HEIGHT))
        .exit_on_esc(true)
        .opengl(OPENGL)
        .build()
        .unwrap();
        let canvas = ImageBuffer::new(WIDTH, HEIGHT);
        let texture = Texture::from_image(
            &canvas,
            &TextureSettings::new());
        App {
            window: window,
            texture: texture
        }
    }


    pub fn init(&mut self, canvas: Arc<Mutex<ImageBuffer<Rgba<u8>, Vec<u8>>>>) {
        println!("Initialized window");
        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                let mut gl = GlGraphics::new(OPENGL);
                let img = Image::new();
                let canvas = canvas.lock().unwrap();
                self.texture.update(&canvas);
                gl.draw(r.viewport(), |c, gl| {
                    clear([1.0; 4], gl);
                    img.draw(&self.texture, &c.draw_state, c.transform, gl);
                });
            }
        }
    }
}