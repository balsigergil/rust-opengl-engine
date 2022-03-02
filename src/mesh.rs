use crate::gl::GLsizei;
use crate::{gl, Ibo, Texture, Vao, Vbo, Vertex};
use std::mem::size_of;
use std::ptr::null;

#[derive(Debug)]
pub struct Mesh {
    vao: Vao,
    ibo: Ibo,
    #[allow(dead_code)]
    vbo: Vbo,
    textures: Vec<Texture>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let vbo = Vbo::new(&vertices);
        let ibo = Ibo::new(&indices);

        let vao = Vao::new()
            .with_layout(0, 3, gl::FLOAT, 0)
            .with_layout(1, 3, gl::FLOAT, 3 * size_of::<f32>())
            .with_layout(2, 3, gl::FLOAT, 6 * size_of::<f32>())
            .with_layout(3, 2, gl::FLOAT, 9 * size_of::<f32>())
            .with_vbo(&vbo, size_of::<Vertex>() as _)
            .with_ibo(&ibo);

        Mesh {
            vao,
            ibo,
            vbo,
            textures,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();
        for texture in &self.textures {
            texture.bind();
        }
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ibo.count() as GLsizei,
                gl::UNSIGNED_INT,
                null(),
            );
        }
        for texture in &self.textures {
            texture.unbind();
        }
        self.vao.unbind();
    }
}
