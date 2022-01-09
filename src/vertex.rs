use glam::{Vec2, Vec3};

#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normals: Vec3,
    pub color: Vec3,
    pub texture_coordinates: Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Default::default(),
            normals: Default::default(),
            color: Default::default(),
            texture_coordinates: Default::default(),
        }
    }
}
