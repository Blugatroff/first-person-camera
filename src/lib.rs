use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Controls {
    pub forwards_backwards: f32,
    pub right_left: f32,
    pub up_down: f32,
    pub look_up_down: f32,
    pub look_right_left: f32,
    pub speed: f32,
}

pub struct FirstPersonCamera {
    position: Vector3<f32>,
    yaw: f32,
    pitch: f32,
}

impl FirstPersonCamera {
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>) -> Self {
        let yaw = direction.x.atan2(direction.z);
        let flat_length = (direction.x.powi(2) + direction.z.powi(2)).sqrt();
        let pitch = flat_length.atan2(yaw);
        Self {
            position,
            yaw,
            pitch,
        }
    }
    pub fn update(&mut self, controls: &Controls) {
        self.yaw += controls.look_right_left;
        self.pitch += controls.look_up_down;

        self.pitch = if self.pitch < -PI / 2.0 + 0.005 {
            -PI / 2.0 + 0.005
        } else if self.pitch > PI / 2.0 - 0.005 {
            PI / 2.0 - 0.005
        } else {
            self.pitch
        };
        let direction = self.get_direction();

        let plane_direction = Vector3::new(direction.x, 0.0, direction.z).normalize();
        let right = Vector3::new(
            (self.yaw - PI / 2.0).sin(),
            0.0,
            (self.yaw - PI / 2.0).cos(),
        )
        .normalize();

        self.position += plane_direction * controls.forwards_backwards * controls.speed;
        self.position += right * controls.right_left * controls.speed;
        self.position += Vector3::unit_y() * controls.up_down * controls.speed;
    }
    pub fn get_direction(&self) -> Vector3<f32> {
        Vector3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }
    pub fn create_view_projection_matrix(
        &self,
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> cgmath::Matrix4<f32> {
        cgmath::perspective(Rad(fov), aspect, near, far)
            * Matrix4::look_at_rh(
                Point3::new(self.position.x, self.position.y, self.position.z),
                {
                    let t = self.position + self.get_direction();
                    Point3::new(t.x, t.y, t.z)
                },
                Vector3::unit_y(),
            )
    }
    pub const fn yaw(&self) -> f32 {
        self.yaw
    }
    pub const fn pitch(&self) -> f32 {
        self.pitch
    }
}
