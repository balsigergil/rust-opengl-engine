use crate::{Camera, Mesh, Shader, Vertex};
use glam::{Mat4, Vec3};
use std::path::Path;

pub struct PointLight {
    mesh: Mesh,
    shader: Shader,
    pub position: Vec3,
    scale: Vec3,
}

impl PointLight {
    pub fn new() -> Self {
        let vertices = vec![
            Vertex {
                position: Vec3::new(-0.5, -0.5, -0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, -0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(-0.5, 0.5, -0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(0.5, 0.5, -0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(0.5, 0.5, 0.5),
                ..Vertex::default()
            },
            Vertex {
                position: Vec3::new(-0.5, 0.5, 0.5),
                ..Vertex::default()
            },
        ];
        let indices = vec![
            0, 1, 2, 2, 3, 0, // Face 1
            0, 1, 5, 5, 4, 0, // Face 2
            1, 2, 6, 6, 5, 1, // Face 3
            2, 3, 7, 7, 6, 2, // Face 4
            3, 0, 4, 4, 7, 3, // Face 5
            4, 5, 6, 6, 7, 4, // Face 6
        ];
        let mesh = Mesh::new(vertices, indices, vec![]);

        let shader = Shader::new(
            Path::new("shaders/light.vert"),
            Path::new("shaders/light.frag"),
        );

        PointLight {
            mesh,
            shader,
            position: Vec3::default(),
            scale: Vec3::splat(0.2),
        }
    }

    pub fn draw(&mut self, camera: &Camera) {
        let model = Mat4::from_translation(self.position) * Mat4::from_scale(self.scale);
        let mvp = camera.get_matrix() * model;

        self.shader.bind();
        self.shader.set_uniform_mat4("uMVP", mvp);
        self.mesh.draw();
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
    }
}
