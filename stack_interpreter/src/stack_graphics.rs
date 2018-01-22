extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use std::thread;
use std::sync::{Arc, Mutex};
use self::piston_window::*;
use std::mem;

#[derive(Debug, Clone)]
pub struct StackWindow {
    image_buffer: Arc<Mutex<im::RgbaImage>>,
    name: String,
    width: u32,
    height: u32
}

impl StackWindow {
    
    pub fn new(window_name: String, width: u32, height: u32) -> StackWindow {
        StackWindow {
            image_buffer: Arc::new(Mutex::new(
                im::ImageBuffer::new(width, height))),
            name: window_name,
            width: width,
            height: height
        }
    }

    pub fn init(&self) { 

        let name = self.name.clone();
        let width = self.width;
        let height = self.height;
        let image_buffer = self.image_buffer.clone();

        thread::spawn( move || {

            let opengl = OpenGL::V3_2;
            let mut window: PistonWindow =
                WindowSettings::new(name, (width, height))
                .exit_on_esc(true)
                .opengl(opengl)
                .build()
                .unwrap();
            
            let im_buf_lock = image_buffer.lock().unwrap();

            let mut texture: G2dTexture = Texture::from_image(
                    &mut window.factory,
                    &*im_buf_lock,
                    &TextureSettings::new()
                ).unwrap();

            mem::drop(im_buf_lock);

            while let Some(e) = window.next() {
                if let Some(_) = e.render_args() {
                    let im_buf_lock = image_buffer.lock().unwrap();
                    texture.update(&mut window.encoder, &*im_buf_lock)
                    .unwrap();
                    mem::drop(im_buf_lock);
                    window.draw_2d(&e, |c, g| {
                        clear([0.0; 4], g);
                        image(&texture, c.transform, g);
                    });
                }
            }
        });
    }

    pub fn drawLine(&self, p0: Point, p1: Point, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_line(&mut *canvas.lock().unwrap(), p0, p1, color);
    }


}

fn brezenham_line(canvas:&mut im::RgbaImage, p0: Point, p1: Point, color: im::Rgba<u8>) {
    let delta_x = (p1.x - p0.x) as f32;
    let delta_y = (p1.y - p0.y) as f32;
    let delta_err = f32::abs(delta_y/delta_x);

    let mut error: f32 = 0.0;
    let mut y = p0.y;
    for x in p0.x..p1.x {
        canvas.put_pixel(x, y, color);
        error = error + delta_err;
        while error >= 0.5 {
            y = y + (delta_y.signum() as u32);
            error = error - 1.0;
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32
}