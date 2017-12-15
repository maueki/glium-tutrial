#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    println!("Hello, world!");

    use glium::Display;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display,
                                                 &[
                                                     Vertex { position: [-0.5, -0.5] },
                                                     Vertex { position: [0.0, 0.5] },
                                                     Vertex { position: [0.5, -0.25] },
                                                 ]).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 110
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 110
        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                _ => (),
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        glutin::ControlFlow::Continue
    });

}
