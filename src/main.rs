#[macro_use]
extern crate glium;

use glium::{glutin};
use glium::index::PrimitiveType;

fn main() {
    println!("Hello, world!");

    use glium::Display;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });

}
