use glam::{Mat4, Vec3};
use glutin::dpi::PhysicalPosition;
use glutin::event::VirtualKeyCode;
use std::time::Duration;

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    orientation: Vec3,
    up: Vec3,
    projection: Mat4,
    pitch: f32,
    yaw: f32,
    speed: f32,
    sensitivity: f32,
    width: u32,
    height: u32,
    fov: f32,
}

impl Camera {
    pub fn new(fov: f32, position: Vec3, width: u32, height: u32) -> Self {
        Camera {
            position,
            projection: Mat4::perspective_rh(fov, width as f32 / height as f32, 0.1, 100.0),
            width,
            height,
            orientation: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            speed: 1.0,
            sensitivity: 0.1,
            pitch: 0.0,
            yaw: 180.0,
            fov,
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.position, self.position + self.orientation, self.up);
        return self.projection * view;
    }

    pub fn update_orientation(&mut self, position: PhysicalPosition<f64>) {
        let delta_x = self.sensitivity * ((self.width as f32 / 2.0) - position.x as f32);
        let delta_y = self.sensitivity * ((self.height as f32 / 2.0) - position.y as f32);

        self.yaw += delta_x;
        self.pitch += delta_y;

        if self.pitch > 89.9 {
            self.pitch = 89.9
        }
        if self.pitch < -89.9 {
            self.pitch = -89.9
        }

        self.orientation = Vec3::new(
            self.pitch.to_radians().cos() * self.yaw.to_radians().sin(),
            self.pitch.to_radians().sin(),
            self.pitch.to_radians().cos() * self.yaw.to_radians().cos(),
        )
        .normalize();
    }

    pub fn update_position(&mut self, inputs: &[bool], delta_time: Duration) {
        if inputs[VirtualKeyCode::W as usize] {
            self.position += self.speed * delta_time.as_secs_f32() * self.orientation;
        }
        if inputs[VirtualKeyCode::S as usize] {
            self.position -= self.speed * delta_time.as_secs_f32() * self.orientation;
        }
        if inputs[VirtualKeyCode::A as usize] {
            self.position -=
                self.speed * delta_time.as_secs_f32() * self.orientation.cross(self.up).normalize();
        }
        if inputs[VirtualKeyCode::D as usize] {
            self.position +=
                self.speed * delta_time.as_secs_f32() * self.orientation.cross(self.up).normalize();
        }
        if inputs[VirtualKeyCode::E as usize] {
            self.position.y += self.speed * delta_time.as_secs_f32();
        }
        if inputs[VirtualKeyCode::Q as usize] {
            self.position.y -= self.speed * delta_time.as_secs_f32();
        }
    }

    pub fn update_viewport(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.projection = Mat4::perspective_rh(self.fov, width as f32 / height as f32, 0.1, 100.0);
    }
}
