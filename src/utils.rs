use log::info;

use std::ffi::CStr;

use crate::gl;

pub fn print_debug_infos() {
    unsafe {
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
        info!(
            "Shading language version: {}",
            CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const _)
                .to_string_lossy()
        );
        let mut mask = 0;
        gl::GetIntegerv(gl::CONTEXT_PROFILE_MASK, &mut mask);
        info!(
            "OpenGL profile: {}",
            if mask == gl::CONTEXT_CORE_PROFILE_BIT as i32 {
                "core"
            } else if mask == gl::CONTEXT_COMPATIBILITY_PROFILE_BIT as i32 {
                "compatibility"
            } else {
                "unknown"
            }
        );
    }
}
