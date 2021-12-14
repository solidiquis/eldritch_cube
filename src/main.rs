use glium::backend::glutin::Display;
use glium::draw_parameters::{Depth, DepthTest, DrawParameters};
use glium::index::{IndexBuffer, PrimitiveType};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::{implement_vertex, uniform, Program, Surface, VertexBuffer};
use nalgebra_glm as glm;
use std::error::Error;
use std::f32::consts::PI;
use std::time::{Instant, Duration};

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;
const DTHETA: f32 = 0.02;
const PI2: f32 = PI * 2.0;

#[derive(Copy, Clone)]
struct Vertex {
    coord: [f32; 3],
    rgba: [f32; 4],
}

implement_vertex!(Vertex, coord, rgba);

fn main() -> Result<(), Box<dyn Error>> {
    let context = ContextBuilder::new()
        .with_depth_buffer(24);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Eldritch Cube")
        .with_inner_size(LogicalSize::new(WIN_WIDTH, WIN_HEIGHT));

    let display = Display::new(window, context, &event_loop)?;

    let cube: [Vertex; 8] = [
        // Front face
        Vertex { coord: [0.5, 0.5, 0.5], rgba: [1.0, 0.0, 0.0, 1.0] },
        Vertex { coord: [0.5, -0.5, 0.5], rgba: [1.0, 0.0, 0.0, 1.0] },
        Vertex { coord: [-0.5, -0.5, 0.5], rgba: [1.0, 0.0, 0.0, 1.0] },
        Vertex { coord: [-0.5, 0.5, 0.5], rgba: [1.0, 0.0, 0.0, 1.0] },

        // Back face
        Vertex { coord: [0.5, 0.5, -0.5], rgba: [0.0, 0.0, 1.0, 1.0] },
        Vertex { coord: [0.5, -0.5, -0.5], rgba: [0.0, 0.0, 1.0, 1.0] },
        Vertex { coord: [-0.5, -0.5, -0.5], rgba: [0.0, 0.0, 1.0, 1.0] },
        Vertex { coord: [-0.5, 0.5, -0.5], rgba: [0.0, 0.0, 1.0, 1.0] },
    ];

    let indices: [u8; 36] = [
        0, 1, 2, 0, 2, 3,
        0, 1, 5, 0, 4, 5,
        2, 3, 6, 3, 6, 7,
        5, 6, 7, 4, 5, 7,
        0, 3, 7, 0, 4, 7,
        1, 2, 6, 1, 5, 6
    ];

    let vbo = VertexBuffer::new(&display, &cube)?;
    let ibuf = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)?;

    let vert_shader_glsl = r#"
        #version 330 core
        layout (location = 0) in vec3 coord;

        uniform mat4 m;
        uniform mat4 v;
        uniform mat4 p;

        in vec4 rgba;
        out vec4 color;

        void main() {
            gl_Position = p * v * m * vec4(coord, 1.0);
            color = rgba;
        }
    "#;

    let frag_shader_glsl = r#"
        # version 150

        in vec4 color;
        out vec4 FragColor;

        void main() {
            FragColor = color;
        }
    "#;

    let program = Program::from_source(
        &display,
        vert_shader_glsl,
        frag_shader_glsl,
        None
    )?;

    let draw_params = DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let v_matrix = glm::translate(
        &glm::TMat4::identity(),
        &glm::vec3(0.0, 0.0, -3.5)
    );

    let p_matrix = glm::perspective(WIN_WIDTH / WIN_HEIGHT, PI / 4.0, 0.1, 100.0);

    let view: [[f32; 4]; 4] = *v_matrix.as_ref();
    let projection: [[f32; 4]; 4] = *p_matrix.as_ref();

    let mut theta = 0.0;

    event_loop.run(move |event, _, ctrlflow| {
        let time_to_next_frame = Instant::now() + Duration::from_nanos(16_666_667);
        *ctrlflow = ControlFlow::WaitUntil(time_to_next_frame);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *ctrlflow = ControlFlow::Exit;
                    return;
                },
                _ => (),
            },

            Event::NewEvents(c) => match c {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },

            _ => return,
        }

        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let m_matrix = glm::rotate(
            &glm::TMat4::identity(),
            theta,
            &glm::vec3(0.0, 1.0, -1.0)
        );

        let model: [[f32; 4]; 4] = *m_matrix.as_ref();
        let uniforms = uniform! { m: model, v: view, p: projection };

        target.draw(
            &vbo, &ibuf, &program,
            &uniforms, &draw_params
        ).unwrap();

        target.finish().unwrap();

        theta = (theta + DTHETA) % PI2;
    });
}
