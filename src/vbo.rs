use crate::gl;
use crate::vertex::Vertex;
use std::mem::size_of;

#[derive(Debug)]
pub struct Vbo {
    pub id: u32,
}

impl Vbo {
    pub fn new(vertices: &[Vertex]) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                (vertices.len() * size_of::<Vertex>()) as _,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
        Vbo { id }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
