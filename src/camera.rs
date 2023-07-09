use crate::settings::Parametros;
use nalgebra_glm::*;

pub struct Camera {
    position: TVec3<f32>,
    yaw: f32,
    pitch: f32,
    up: TVec3<f32>,
    right: TVec3<f32>,
    forward: TVec3<f32>,
    // Matrices
    pub m_proj: TMat4<f32>,
    pub m_view: TMat4<f32>,
}

impl Camera {
    pub fn new(position: TVec3<f32>, yaw: f32, pitch: f32, param: &Parametros) -> Camera {
        Camera {
            //parametros: param,
            position,
            yaw,
            pitch,
            up: TVec3::new(0., 1., 0.),
            right: TVec3::new(1., 0., 0.),
            forward: TVec3::new(0., 0., -1.),
            m_proj: perspective(param.v_fov, param.aspect_ratio, param.near, param.far),
            // m_view: TMat4::new(1., 0., 0., 0.,
            //                    0., 1., 0., 0.,
            //                    0., 0., 1., 0.,
            //                    0., 0., 0., 1.),
            //m_proj: TMat4::identity(),
            m_view: TMat4::identity(),
        }
    }

    pub fn update(&mut self) {
        self.update_vectors();
        self.update_view_matrix();
    }

    pub fn update_view_matrix(&mut self) {
        self.m_view = look_at(&self.position, &(self.position + self.forward), &self.up);
    }

    pub fn update_vectors(&mut self) {
        self.forward.x = self.yaw.cos() * self.pitch.cos();
        self.forward.y = self.pitch.sin();
        self.forward.z = self.yaw.sin() * self.pitch.cos();

        self.forward = normalize(&self.forward);
        self.right = normalize(&(cross(&self.forward, &TVec3::new(0., 1., 0.))));
        self.up = normalize(&cross(&self.right, &self.forward));
    }

    pub fn rotate_pitch(&mut self, delta_y: f32, param: &Parametros) {
        self.pitch -= delta_y;
        self.pitch = clamp_scalar(self.pitch, -param.pitch_max, param.pitch_max);
    }

    pub fn rotate_yaw(&mut self, delta_x: f32) {
        self.yaw += delta_x;
    }

    pub fn move_left(&mut self, velocity: f32) {
        self.position -= self.right * velocity;
    }

    pub fn move_right(&mut self, velocity: f32) {
        self.position += self.right * velocity;
    }

    pub fn move_up(&mut self, velocity: f32) {
        self.position += self.up * velocity;
    }

    pub fn move_down(&mut self, velocity: f32) {
        self.position -= self.up * velocity;
    }

    pub fn move_forward(&mut self, velocity: f32) {
        self.position += self.forward * velocity;
    }

    pub fn move_back(&mut self, velocity: f32) {
        self.position -= self.forward * velocity;
    }
}