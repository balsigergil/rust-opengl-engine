use crate::camera::Camera;
use crate::ibo::Ibo;
use crate::mesh::Mesh;
use crate::point_light::PointLight;
use crate::shader::Shader;
use crate::texture::{Texture, TextureKind};
use crate::utils::print_debug_infos;
use crate::vao::Vao;
use crate::vbo::Vbo;
use crate::vertex::Vertex;
use glad::gl;
use glam::{Mat4, Vec2, Vec3};
use glutin::dpi::PhysicalPosition;
use glutin::event::{ElementState, MouseButton};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlProfile, GlRequest,
};
use log::{error, info, trace, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null;
use std::time::Instant;

mod camera;
mod glad;
mod ibo;
mod mesh;
mod model;
mod point_light;
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

    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Rust OpenGL engine v0.0.1 (x64)")
        .with_visible(false)
        .with_inner_size(glutin::dpi::PhysicalSize::new(WIDTH, HEIGHT));

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 6)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load(|s| windowed_context.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(debug_callback, null());
        gl::ClearColor(0.2, 0.3, 0.8, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    print_debug_infos();

    let vertices = vec![
        Vertex {
            position: Vec3::new(-2.0, 0.0, -2.0),
            normals: Vec3::new(0.0, 1.0, 0.0),
            color: Default::default(),
            texture_coordinates: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(2.0, 0.0, -2.0),
            normals: Vec3::new(0.0, 1.0, 0.0),
            color: Default::default(),
            texture_coordinates: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(2.0, 0.0, 2.0),
            normals: Vec3::new(0.0, 1.0, 0.0),
            color: Default::default(),
            texture_coordinates: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-2.0, 0.0, 2.0),
            normals: Vec3::new(0.0, 1.0, 0.0),
            color: Default::default(),
            texture_coordinates: Vec2::new(0.0, 1.0),
        },
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    let planks_diffuse = Texture::new(
        Path::new("res/wood_floor/WoodFlooring044_COL_1K.jpg"),
        TextureKind::DIFFUSE,
    );
    let planks_specular = Texture::new(
        Path::new("res/wood_floor/WoodFlooring044_REFL_1K.jpg"),
        TextureKind::SPECULAR,
    );
    let mesh = Mesh::new(vertices, indices, vec![planks_diffuse, planks_specular]);

    let mut shader = Shader::new(
        Path::new("shaders/default.vert"),
        Path::new("shaders/default.frag"),
    );

    shader.bind();
    shader.set_uniform_1i("uTextureDiffuse", 0);
    shader.set_uniform_1i("uTextureSpecular", 1);

    let mut camera = Camera::new(45.0, Vec3::new(0.0, 1.0, 1.0), WIDTH, HEIGHT);

    let mut point_light = PointLight::new();
    point_light.set_position(Vec3::new(0.0, 0.3, 0.0));
    shader.set_uniform_vec3("uLightPosition", point_light.position);
    shader.set_uniform_vec3("uLightColor", Vec3::new(1.0, 1.0, 1.0));

    let mut fps_timer = Instant::now();
    let mut counter = 0;

    let mut last_time = Instant::now();

    let mut inputs = [false; 512];

    let mut mouse_captured = false;

    windowed_context.window().set_visible(true);

    el.run(move |e, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    let window_size = windowed_context.window().inner_size();
                    camera.update_viewport(window_size.width, window_size.height);
                    unsafe {
                        gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
                    }
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        inputs[keycode as usize] = input.state == ElementState::Pressed;
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        windowed_context.window().set_cursor_visible(false);
                        let window_size = windowed_context.window().inner_size();
                        windowed_context
                            .window()
                            .set_cursor_position(PhysicalPosition::new(
                                window_size.width as f64 / 2.0,
                                window_size.height as f64 / 2.0,
                            ))
                            .unwrap();
                        mouse_captured = true;
                    } else {
                        windowed_context.window().set_cursor_visible(true);
                        mouse_captured = false;
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if mouse_captured {
                        camera.update_orientation(position);
                        let window_size = windowed_context.window().inner_size();
                        windowed_context
                            .window()
                            .set_cursor_position(PhysicalPosition::new(
                                window_size.width as f64 / 2.0,
                                window_size.height as f64 / 2.0,
                            ))
                            .unwrap();
                    }
                }
                _ => (),
            },
            Event::RedrawEventsCleared => windowed_context.window().request_redraw(),
            Event::RedrawRequested(_) => {
                let delta_time = last_time.elapsed();
                last_time = Instant::now();

                if fps_timer.elapsed().as_millis() < 1000 {
                    counter += 1;
                } else {
                    info!("FPS: {counter}");
                    fps_timer = Instant::now();
                    counter = 0;
                }

                camera.update_position(&inputs, delta_time);

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }

                let model = Mat4::IDENTITY;

                shader.bind();
                shader.set_uniform_mat4("uCameraViewProjection", camera.get_matrix());
                shader.set_uniform_mat4("uModel", model);
                shader.set_uniform_vec3("uCameraPosition", camera.position);
                mesh.draw();
                shader.unbind();

                point_light.draw(&camera);

                windowed_context.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => return,
            _ => (),
        };
    })
}
