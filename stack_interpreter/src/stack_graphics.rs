extern crate piston_window;
extern crate image as im;
extern crate vecmath;
extern crate rusttype;

use std::thread;
use std::sync::{Arc, Mutex};
use self::piston_window::*;
use std::mem;
use self::rusttype::{FontCollection, Scale, Font};

#[derive(Clone)]
pub struct StackWindow<'a> {
    image_buffer: Arc<Mutex<im::RgbaImage>>,
    name: String,
    width: u32,
    height: u32,
    font: Font<'a>
}

impl<'a> StackWindow<'a> {
    
    pub fn new(window_name: String, width: u32, height: u32) -> StackWindow<'a> { 
        // Load the font
        let font_data = include_bytes!("../fonts/typewriter/TYPEWR__.TTF");
        let collection = FontCollection::from_bytes(font_data as &[u8]);
        // This only succeeds if collection consists of one font
        let font = collection.into_font().unwrap();

        StackWindow {
            image_buffer: Arc::new(Mutex::new(
                im::ImageBuffer::new(width, height))),
            name: window_name,
            width: width,
            height: height,
            font: font
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

    // draw a line between the given vertices using color
    pub fn draw_line(&self, p0: Point, p1: Point, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_line(&mut *canvas.lock().unwrap(), p0, p1, color);
    }

    // draw a triangle with given vertices and color
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

    // draw a circle with given center point, radius and color
    pub fn draw_circle(&self, p0: Point, r: u32, color: im::Rgba<u8>) {
        let canvas = self.image_buffer.clone();
        brezenham_circle(&mut *canvas.lock().unwrap(), p0, r, color);
    }

    // clear canvas with transparent black
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

    pub fn draw_text(&self) {
        let image_buffer = self.image_buffer.clone();
        let mut canvas = image_buffer.lock().unwrap();
        rasterize_text(&mut canvas, &self.font, Point{ x:0, y:0 }, im::Rgba([150, 0, 0, 255]));
    }

}

// use brezenham's line algorithm
fn brezenham_line(canvas:&mut im::RgbaImage, p0: Point, p1: Point, color: im::Rgba<u8>) {
    let delta_x = (p1.x as i32 - p0.x as i32) as f32;
    let delta_y = (p1.y as i32 - p0.y as i32) as f32;
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

// use brezenham's circle algorithm
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

fn rasterize_text(canvas: &mut im::RgbaImage, font: &Font, start: Point, color: im::Rgba<u8>) {

    // The font size to use
    let size = 3.0;
    let scale = Scale {x: size, y: size};

    // The text to render
    let text = "This is RustType rendered into a png!";

    // Transform p into rusttype Point
    let start = rusttype::point(start.x as f32, start.y as f32);

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in font.layout(text, scale, start) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, _v| canvas.put_pixel(
                // Offset the position by the glyph bounding box
                x + bounding_box.min.x as u32,
                y + bounding_box.min.y as u32,
                // Turn the coverage into color
                color
            ));
        }
    }
}

#[derive(Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32
}
