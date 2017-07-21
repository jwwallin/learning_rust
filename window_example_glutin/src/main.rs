extern crate gl;
extern crate glutin;
extern crate libc;
extern crate time;

use glutin::GlContext;
use gl::types::*;

static WINDOW_TITLE: &str = "Hello, World";
static WINDOW_WIDTH: u32 = 1024;
static WINDOW_HEIGHT: u32 = 768;


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
                    .with_title(WINDOW_TITLE)
                    .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                    .with_min_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                    .with_max_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);
    let context = glutin::ContextBuilder::new()
                    .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop)
                    .unwrap();
    println!("Window created");

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }
    println!("Function addresses loaded");

    let mut time_previous = time::PreciseTime::now();
    let mut running = true;
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}
