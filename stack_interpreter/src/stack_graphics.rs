extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use std::thread;
use self::piston_window::*;

pub struct StackWindow {
    image_buffer: im::RgbaImage,
    name: String,
    width: u32,
    height: u32
}

impl StackWindow {
    
    pub fn new(window_name: String, width: u32, height: u32) -> StackWindow {
        StackWindow {
            image_buffer: im::ImageBuffer::new(width, height),
            name: window_name,
            width: width,
            height: height
        }
    }

    pub fn init(self) { 
        thread::spawn(move || {

            let opengl = OpenGL::V3_2;
            let mut window: PistonWindow =
                WindowSettings::new(self.name, (self.width, self.height))
                .exit_on_esc(true)
                .opengl(opengl)
                .build()
                .unwrap();
                
            let mut texture: G2dTexture = Texture::from_image(
                    &mut window.factory,
                    &self.image_buffer,
                    &TextureSettings::new()
                ).unwrap();

            while let Some(e) = window.next() {
                if let Some(_) = e.render_args() {
                    texture.update(&mut window.encoder, &self.image_buffer).unwrap();
                    window.draw_2d(&e, |c, g| {
                        clear([0.0; 4], g);

                        image(&texture, c.transform, g);
                    });
                }
            }
        });
    }
}