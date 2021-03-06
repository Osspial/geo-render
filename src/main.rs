extern crate gl_raii;
#[macro_use]
extern crate gl_raii_macros;
extern crate cgmath;
extern crate cgmath_geometry;
extern crate glutin;

use gl_raii::ContextState;
use gl_raii::buffers::*;
use gl_raii::framebuffer::*;
use gl_raii::program::*;
use gl_raii::vao::*;
use gl_raii::glsl::*;
use gl_raii::colors::*;
use gl_raii::render_state::*;

use cgmath_geometry::*;

use cgmath::*;

use glutin::{GlContext, EventsLoop, Event, WindowEvent, ControlFlow, WindowBuilder, ContextBuilder, GlWindow};

#[derive(TypeGroup, Clone, Copy)]
struct Vertex {
    pos: Point2<f32>,
    color: Rgb<Nu8>
}

#[derive(Clone, Copy, Uniforms)]
struct TriUniforms {
    offset: Point2<u32>
}

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = GlWindow::new(
        WindowBuilder::new().with_dimensions(512, 512),
        ContextBuilder::new().with_srgb(true),
        &events_loop
    ).unwrap();
    unsafe{ window.context().make_current().unwrap() };
    let state = unsafe{ ContextState::new(|addr| window.context().get_proc_address(addr)) };

    let la = Segment::new2(0.0, 0.0, 1.0, 1.0);
    let lb = Segment::new2(0.0, 1.0, 1.0, 0.0);
    let inter = Into::<Option<_>>::into(la.intersect(lb)).unwrap_or(Point2::new(9.0, 9.0));
    println!("{:?}", la.intersect(lb));

    let vertex_buffer = Buffer::with_data(BufferUsage::StaticDraw, &[
        Vertex {
            pos: la.start(),
            color: Rgb::new(Nu8(255), Nu8(0), Nu8(0))
        },
        Vertex {
            pos: la.end(),
            color: Rgb::new(Nu8(255), Nu8(0), Nu8(0))
        },
        Vertex {
            pos: lb.start(),
            color: Rgb::new(Nu8(0), Nu8(255), Nu8(0))
        },
        Vertex {
            pos: lb.end(),
            color: Rgb::new(Nu8(0), Nu8(255), Nu8(0))
        },

        Vertex {
            pos: inter + Vector2::new(0.0, 0.03),
            color: Rgb::new(Nu8(255), Nu8(255), Nu8(0))
        },
        Vertex {
            pos: inter + Vector2::new(0.03, -0.03),
            color: Rgb::new(Nu8(255), Nu8(255), Nu8(0))
        },
        Vertex {
            pos: inter + Vector2::new(-0.03, -0.03),
            color: Rgb::new(Nu8(255), Nu8(255), Nu8(0))
        },
    ], state.clone());
    let vao = VertexArrayObj::new_noindex(vertex_buffer);


    let vertex_shader = Shader::new(VERTEX_SHADER, state.clone()).unwrap();
    let fragment_shader = Shader::new(FRAGMENT_SHADER, state.clone()).unwrap();
    let program = Program::new(&vertex_shader, None, &fragment_shader).unwrap_discard();

    let mut render_state = RenderState {
        srgb: true,
        ..RenderState::default()
    };

    let mut default_framebuffer = DefaultFramebuffer::new(state.clone());
    events_loop.run_forever(|event| {
        match event {
            Event::WindowEvent{event, ..} => match event {
                WindowEvent::Resized(size_x, size_y) => {
                    let uniform = TriUniforms {
                        offset: Point2::new(0, 0)
                    };
                    render_state.viewport = OffsetBox {
                        origin: Point2::new(0, 0),
                        dims: Vector2::new(size_x, size_y)
                    };
                    default_framebuffer.clear_color(Rgba::new(0.0, 0.0, 0.0, 1.0));
                    default_framebuffer.draw(DrawMode::Lines, 0..4, &vao, &program, uniform, render_state);
                    default_framebuffer.draw(DrawMode::Triangles, 4.., &vao, &program, uniform, render_state);

                    window.context().swap_buffers().unwrap();
                }
                WindowEvent::Closed => return ControlFlow::Break,
                _ => ()
            },
            _ => ()
        }

        ControlFlow::Continue
    });
}

const VERTEX_SHADER: &str = r#"
    #version 330

    in vec2 pos;
    in vec3 color;
    out vec3 vert_color;

    uniform uvec2 offset;

    void main() {
        vert_color = color;
        gl_Position = vec4(pos + vec2(offset), 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 330

    in vec3 vert_color;
    out vec4 frag_color;

    void main() {
        frag_color = vec4(vert_color, 1.0);
    }
"#;
