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
        let vao = Vao::new();
        vao.bind();
        let vbo = Vbo::new(&vertices);
        vbo.bind();
        let ibo = Ibo::new(&indices);
        ibo.bind();

        vao.add_layout(0, 3, gl::FLOAT, size_of::<Vertex>(), 0);
        vao.add_layout(1, 3, gl::FLOAT, size_of::<Vertex>(), 3 * size_of::<f32>());
        vao.add_layout(2, 3, gl::FLOAT, size_of::<Vertex>(), 6 * size_of::<f32>());
        vao.add_layout(3, 2, gl::FLOAT, size_of::<Vertex>(), 9 * size_of::<f32>());

        vao.unbind();
        ibo.unbind();
        vbo.unbind();

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
