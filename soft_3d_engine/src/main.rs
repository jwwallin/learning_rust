extern crate gl;
extern crate glutin;
extern crate time;
extern crate rusttype;

use gl::types::*;
use glutin::GlContext;
use rusttype::{FontCollection, Scale, point, PositionedGlyph};

mod open_gl_shaders;

static WINDOW_TITLE: &str = "Enter Title Here";
static WINDOW_WIDTH: u32 = 1024;
static WINDOW_HEIGHT: u32 = 768;

fn main() {
    // initialize font
    let font_data = include_bytes!("Arial Unicode.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap();

    // font pixel height
    let height: f32 = 12.4;
    let pixel_height = height.ceil() as usize;

    // scale in x direction to counter aspect ratio of monospace font
    let scale = Scale {x: height*2.0, y: height};

    // initialize window
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_max_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_min_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window_config, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    // initialize opengl bindings
    unsafe {
        gl::load_with(|s| gl_window.get_proc_address(s) as *const _);
    }

    // initialize shaders and setup blending for text rendering
    unsafe {
        let font_raster_shdr = open_gl_shaders::compile_shader(open_gl_shaders::SHDR_GLYPH_RASTERIZER, gl::VERTEX_SHADER);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    // main render loop
    let mut running = true;
    let mut time_previousFrame = time::PreciseTime::now();
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    _ => (),
                },
                _ => (),
            }
        });

        // calculate fps
        let time_currentFrame = time::PreciseTime::now();
        let frame_time = time_previousFrame.to(time_currentFrame).num_microseconds().unwrap_or(1_000_000_000);
        time_previousFrame = time_currentFrame.clone();
        let frame_rate = 1_000_000/frame_time;
        let FPS_display = format!("FPS: {}    FrameTime: {}", frame_rate, frame_time);

        // render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        let (mut tex, mut uniform_tex): (GLuint, GLint) = (0, 0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::Uniform1i(uniform_tex, 0);
        }
        gl_window.swap_buffers().unwrap();
    }
    
}
