use glam::Vec3;
use std::ffi::{c_void, CStr};
use std::mem::size_of;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null;

use glad::gl;

use crate::gl::MAX_HEIGHT;
use crate::ibo::Ibo;
use crate::shader::Shader;
use crate::vao::Vao;
use crate::vbo::Vbo;
use crate::vertex::Vertex;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use log::{error, info, trace, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

mod camera;
mod glad;
mod ibo;
mod mesh;
mod model;
mod shader;
mod texture;
mod vao;
mod vbo;
mod vertex;

extern "system" fn debug_callback(
    source: u32,
    message_type: u32,
    id: u32,
    severity: u32,
    length: i32,
    message: *const c_char,
    userParam: *mut c_void,
) {
    unsafe {
        if message_type == gl::DEBUG_TYPE_ERROR {
            error!("GL Error: {:?}", CStr::from_ptr(message));
        } else {
            trace!("{:?}", CStr::from_ptr(message));
        }
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::default().build(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();

    const WIDTH: u32 = 1270;
    const HEIGHT: u32 = 720;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Rust OpenGL engine v0.0.1 (x64)")
        .with_resizable(false)
        .with_visible(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load(|s| windowed_context.get_proc_address(s));

    windowed_context.window().set_visible(true);

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(debug_callback, null());
        gl::ClearColor(0.2, 0.3, 0.8, 1.0);

        info!(
            "OpenGL version : {}",
            CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_string_lossy()
        );
        info!(
            "OpenGL vendor  : {}",
            CStr::from_ptr(gl::GetString(gl::VENDOR) as *const _).to_string_lossy()
        );
        info!(
            "OpenGL renderer: {}",
            CStr::from_ptr(gl::GetString(gl::RENDERER) as *const _).to_string_lossy()
        );
    }

    let vertices = vec![
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Default::default(),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Default::default(),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Default::default(),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Default::default(),
        },
    ];

    let vao = Vao::new();
    let vbo = Vbo::new(&vertices);
    let ibo = Ibo::new(&[0, 1, 2, 2, 3, 0]);
    vao.add_layout(0, 3, gl::FLOAT, size_of::<Vertex>(), 0);
    vao.add_layout(1, 3, gl::FLOAT, size_of::<Vertex>(), 3 * size_of::<f32>());
    vao.add_layout(2, 3, gl::FLOAT, size_of::<Vertex>(), 6 * size_of::<f32>());
    vao.add_layout(3, 2, gl::FLOAT, size_of::<Vertex>(), 9 * size_of::<f32>());

    vbo.unbind();
    vao.unbind();
    ibo.unbind();

    let shader = Shader::new(
        Path::new("shaders/default.vert"),
        Path::new("shaders/default.frag"),
    );

    shader.bind();
    vao.bind();

    el.run(move |e, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                unsafe {
                    gl::DrawElements(gl::TRIANGLES, ibo.count(), gl::UNSIGNED_INT, null());
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        };
    })
}
