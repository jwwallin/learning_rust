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

    pub fn draw_line(&self, p0: Point, p1: Point, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_line(&mut *canvas.lock().unwrap(), p0, p1, color);
    }

    pub fn draw_triangle(
        &self, p0: Point, p1: Point, p2: Point, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_line(&mut *canvas.lock().unwrap(), 
            p0.clone(), p1.clone(), color);
        brezenham_line(&mut *canvas.lock().unwrap(), 
            p1.clone(), p2.clone(), color);
        brezenham_line(&mut *canvas.lock().unwrap(), 
            p0.clone(), p2.clone(), color);
    }

    pub fn draw_circle(&self, p0: Point, r: u32, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_circle(&mut *canvas.lock().unwrap(), p0, r, color);
    }

    pub fn clear_canvas(&self) {
        let image_buffer = self.image_buffer.clone();
        let mut canvas = image_buffer.lock().unwrap();
        let (width, height) = canvas.dimensions();
        for x in 0..width {
            for y in 0..height {
                canvas.put_pixel(x, y, im::Rgba([0,0,0,0]));
            }
        }
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

fn brezenham_circle(canvas:&mut im::RgbaImage, p0: Point, r: u32, color: im::Rgba<u8>) {
    
    let x0: u32 = p0.x;
    let y0:u32 = p0.y;
    let mut x: u32 = r - 1;
    let mut y: u32 = 0;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;
    let mut err = dx - (r * 2) as i32;

    while x >= y {
        
        canvas.put_pixel(x0 + x, y0 + y, color);
        canvas.put_pixel(x0 + y, y0 + x, color);
        canvas.put_pixel(x0 - y, y0 + x, color);
        canvas.put_pixel(x0 - x, y0 + y, color);
        canvas.put_pixel(x0 - x, y0 - y, color);
        canvas.put_pixel(x0 - y, y0 - x, color);
        canvas.put_pixel(x0 + y, y0 - x, color);
        canvas.put_pixel(x0 + x, y0 - y, color);

        if err <= 0 {
            y = y + 1;
            err += dy;
            dy += 2;
        } else {
            x = x - 1;
            dx += 2;
            err += dx - (r * 2) as i32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32
}