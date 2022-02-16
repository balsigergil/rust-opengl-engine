use glam::Mat4;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null;

use log::error;

use crate::gl;
use crate::gl::GLint;

pub struct Shader {
    pub id: u32,
    location_cache: HashMap<String, GLint>,
}

impl Shader {
    pub fn new(vertex_file: &Path, fragment_file: &Path) -> Self {
        let mut vertex_file = File::open(vertex_file).expect("Could not open vertex shader file");
        let mut vertex_source = String::new();
        vertex_file.read_to_string(&mut vertex_source).unwrap();

        let mut fragment_file =
            File::open(fragment_file).expect("Could not open fragment shader file");
        let mut fragment_source = String::new();
        fragment_file.read_to_string(&mut fragment_source).unwrap();

        let program_id;

        unsafe {
            let vert_id = compile_shader(gl::VERTEX_SHADER, vertex_source);
            let frag_id = compile_shader(gl::FRAGMENT_SHADER, fragment_source);
            program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vert_id);
            gl::AttachShader(program_id, frag_id);
            gl::LinkProgram(program_id);
            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);
        }

        Shader {
            id: program_id,
            location_cache: HashMap::new(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform_1_i(&mut self, location: &str, value: i32) {
        unsafe {
            let location_index = self.get_location(location);
            gl::Uniform1i(location_index, value);
        }
    }

    pub fn set_uniform_mat4(&mut self, location: &str, matrix: Mat4) {
        unsafe {
            let location_index = self.get_location(location);
            gl::UniformMatrix4fv(location_index, 1, gl::FALSE, matrix.as_ref().as_ptr());
        }
    }

    fn get_location(&mut self, location: &str) -> GLint {
        unsafe {
            *self
                .location_cache
                .entry(location.to_string())
                .or_insert_with(|| {
                    let location = CString::new(location).unwrap();
                    gl::GetUniformLocation(self.id, location.as_ptr())
                })
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

unsafe fn compile_shader(shader_type: u32, source: String) -> u32 {
    let id = gl::CreateShader(shader_type);
    let raw_source = CString::new(source).unwrap();
    gl::ShaderSource(id, 1, &raw_source.as_ptr() as *const _, null());
    gl::CompileShader(id);
    check_compile_status(id, shader_type);
    return id;
}

unsafe fn check_compile_status(shader: u32, shader_type: u32) {
    let mut result = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);

    if result == 0 {
        let mut length = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
        let mut message: Vec<c_char> = Vec::with_capacity(length as usize);
        gl::GetShaderInfoLog(shader, length, &mut length, message.as_mut_ptr());
        let message = CStr::from_ptr(message.as_ptr()).to_string_lossy();
        error!(
            "Failed to compile {} shader: {}",
            if shader_type == gl::VERTEX_SHADER {
                "vertex"
            } else {
                "fragment"
            },
            message
        );
    }
}
