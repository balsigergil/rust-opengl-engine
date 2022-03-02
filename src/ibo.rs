use crate::gl;
use std::mem::size_of;

#[derive(Debug)]
pub struct Ibo {
    pub id: u32,
    count: usize,
}

impl Ibo {
    pub fn new(indices: &[u32]) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                (indices.len() * size_of::<u32>()) as _,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
        Ibo {
            id,
            count: indices.len(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn count(&self) -> i32 {
        self.count as i32
    }
}

impl Drop for Ibo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
