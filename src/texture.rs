use crate::gl;
use image::EncodableLayout;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum TextureKind {
    DIFFUSE,
    SPECULAR,
}

#[derive(Debug)]
pub struct Texture {
    id: u32,
    kind: TextureKind
}

impl Texture {
    pub fn new(path: &Path, kind: TextureKind) -> Self {
        let image = image::open(path).expect("Unable to open texture").flipv();
        let rgba_image = image.into_rgba8();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            if kind == TextureKind::DIFFUSE {
                gl::ActiveTexture(gl::TEXTURE0);
            } else if kind == TextureKind::SPECULAR {
                gl::ActiveTexture(gl::TEXTURE1);
            }
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                rgba_image.width() as i32,
                rgba_image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba_image.as_bytes().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture { id, kind }
    }

    pub fn bind(&self) {
        unsafe {
            if self.kind == TextureKind::DIFFUSE {
                gl::ActiveTexture(gl::TEXTURE0);
            } else if self.kind == TextureKind::SPECULAR {
                gl::ActiveTexture(gl::TEXTURE1);
            }
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
