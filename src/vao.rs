use crate::gl;

pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
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

    pub fn add_layout(
        &self,
        index: u32,
        num_components: u32,
        component_type: u32,
        stride: usize,
        offset: usize,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                num_components as i32,
                component_type,
                gl::FALSE,
                stride as i32,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(index);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
