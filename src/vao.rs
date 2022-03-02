use std::{any::Any, mem::size_of};

use crate::{gl, ibo::Ibo, vbo::Vbo, vertex::Vertex};

#[derive(Debug)]
pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn with_layout(
        self,
        index: u32,
        num_components: u32,
        component_type: u32,
        offset: usize,
    ) -> Self {
        unsafe {
            gl::VertexArrayAttribFormat(
                self.id,
                index,
                num_components as _,
                component_type,
                gl::FALSE,
                offset as _,
            );
            gl::VertexArrayAttribBinding(self.id, index, 0);
            gl::EnableVertexArrayAttrib(self.id, index);
        }
        return self;
    }

    pub fn with_vbo(self, vbo: &Vbo, stride: i32) -> Self {
        unsafe { gl::VertexArrayVertexBuffer(self.id, 0, vbo.id, 0, stride) }
        self
    }

    pub fn with_ibo(self, ibo: &Ibo) -> Self {
        unsafe { gl::VertexArrayElementBuffer(self.id, ibo.id) }
        self
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
