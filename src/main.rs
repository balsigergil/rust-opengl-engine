use egui::epaint::Shadow;
use egui::{Align, Color32, Direction, Label, Layout, ProgressBar, RichText, Visuals};
use egui_glow::EguiGlow;
use glam::{Mat4, Vec2, Vec3};
use std::ffi::{c_void, CStr, CString};
use std::mem::size_of;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null;
use std::time::Instant;

use glad::gl;

use crate::ibo::Ibo;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::utils::print_debug_infos;
use crate::vao::Vao;
use crate::vbo::Vbo;
use crate::vertex::Vertex;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlProfile, GlRequest,
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
mod utils;
mod vao;
mod vbo;
mod vertex;

extern "system" fn debug_callback(
    _source: u32,
    message_type: u32,
    _id: u32,
    _severity: u32,
    _length: i32,
    message: *const c_char,
    _user_param: *mut c_void,
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
        LevelFilter::Debug,
        ConfigBuilder::default().build(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    )
    .unwrap();

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Rust OpenGL engine v0.0.1 (x64)")
        .with_resizable(false)
        .with_visible(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 6)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load(|s| windowed_context.get_proc_address(s));

    let gl_context =
        unsafe { glow::Context::from_loader_function(|e| windowed_context.get_proc_address(e)) };

    windowed_context.window().set_visible(true);

    let mut egui_glow = EguiGlow::new(&windowed_context, &gl_context);

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(debug_callback, null());
        gl::ClearColor(0.2, 0.3, 0.8, 1.0);
    }

    print_debug_infos();

    let vertices = vec![
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.0),
            normals: Default::default(),
            color: Vec3::new(0.8, 0.3, 0.2),
            texture_coordinates: Vec2::new(0.0, 1.0),
        },
    ];

    let vao = Vao::new();
    vao.bind();
    let vbo = Vbo::new(&vertices);
    vbo.bind();
    let indices = vec![0, 1, 2, 2, 3, 0];
    let ibo = Ibo::new(&indices);
    ibo.bind();

    vao.add_layout(0, 3, gl::FLOAT, size_of::<Vertex>(), 0);
    vao.add_layout(1, 3, gl::FLOAT, size_of::<Vertex>(), 3 * size_of::<f32>());
    vao.add_layout(2, 3, gl::FLOAT, size_of::<Vertex>(), 6 * size_of::<f32>());
    vao.add_layout(3, 2, gl::FLOAT, size_of::<Vertex>(), 9 * size_of::<f32>());

    vao.unbind();
    ibo.unbind();
    vbo.unbind();

    let logo = Texture::new(Path::new("res/logo.png"));
    logo.bind();

    let shader = Shader::new(
        Path::new("shaders/default.vert"),
        Path::new("shaders/default.frag"),
    );

    shader.bind();
    vao.bind();

    unsafe {
        let location = CString::new("uTexture").unwrap();
        gl::Uniform1i(gl::GetUniformLocation(shader.id, location.as_ptr()), 0);
    }

    let mut last_time = Instant::now();
    let mut counter = 0;

    let mut angle = 0.0;

    let mut delta_time = Instant::now();

    let mut quit = false;

    let mut visuals = Visuals::default();
    visuals.window_shadow = Shadow {
        extrusion: 8.0,
        color: Color32::from_black_alpha(96),
    };
    visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));

    egui_glow.egui_ctx.set_visuals(visuals.clone());

    let mut color: [f32; 3] = [0.0, 0.0, 0.0];
    let mut open = true;
    let mut text = String::new();

    el.run(move |e, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if quit {
            *control_flow = ControlFlow::Exit;
        }

        match e {
            Event::WindowEvent { event, .. } => {
                egui_glow.on_event(&event);
                match event {
                    WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                }
                windowed_context.window().request_redraw();
            }
            Event::RedrawEventsCleared => windowed_context.window().request_redraw(),
            Event::RedrawRequested(_) => {
                if last_time.elapsed().as_millis() < 1000 {
                    counter += 1;
                } else {
                    info!("FPS: {}", counter);
                    last_time = Instant::now();
                    counter = 0;
                }
                if delta_time.elapsed().as_secs_f32() > 1.0 / 120.0 {
                    delta_time = Instant::now();
                    angle += 0.01;
                }

                let rotation = Mat4::from_rotation_z(angle);
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    vao.bind();
                    shader.bind();
                    logo.bind();

                    let location = CString::new("rotation").unwrap();
                    gl::UniformMatrix4fv(
                        gl::GetUniformLocation(shader.id, location.as_ptr()),
                        1,
                        gl::FALSE,
                        rotation.as_ref().as_ptr(),
                    );

                    gl::DrawElements(gl::TRIANGLES, ibo.count(), gl::UNSIGNED_INT, null());
                }

                let (needs_repaint, shapes) =
                    egui_glow.run(windowed_context.window(), |egui_ctx| {
                        egui::Window::new("Hello World")
                            .resizable(true)
                            .default_size((60.0, 40.0))
                            .show(egui_ctx, |ui| {
                                if ui.button("Quit").clicked() {
                                    quit = true;
                                }
                                ui.allocate_space(ui.available_size());
                            });
                    });

                if needs_repaint {
                    windowed_context.window().request_redraw();
                }

                egui_glow.paint(&windowed_context, &gl_context, shapes);

                windowed_context.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => egui_glow.destroy(&gl_context),
            _ => (),
        };
    })
}
