use std::time::Instant;
use crate::camera::Camera;
use crate::settings::Parametros;
use nalgebra_glm::*;
use crate::VoxelEngine;

pub struct Player {
    //app: VoxelEngine,
    pub camera: Camera,
}

impl Player {
    pub fn new(param: &Parametros) -> Player {
        let position = param.player_pos;
        let yaw = -90.;
        let pitch = 0.;
        let camera = Camera::new(position, yaw, pitch, param);
        Player {
            //app,
            camera,
        }
    }

    pub fn update(&mut self, clock: Instant, param: &mut Parametros) {
        self.keyboard_control(param, clock);
        self.mouse_control(param);
        self.camera.update();
    }

    pub fn mouse_control(&mut self, param: &mut Parametros) {
        //if param.raton.xrel > 0 {
        // TODO esto es provisional

        let delta_x = param.raton.xrel as f32 * param.mouse_sensivity;
        //dbg!(&delta_x);
        self.camera.rotate_yaw(delta_x);
        //}

        //if param.raton.yrel > 0 {
        // TODO esto es provisional
        let delta_y = param.raton.yrel as f32 * param.mouse_sensivity;
        self.camera.rotate_pitch(delta_y, param);
        //}
    }

    pub fn keyboard_control(&mut self, param: &Parametros, clock: Instant) {
        // TODO esto es provisional
        let vel = param.player_speed * clock.elapsed().as_millis() as f32 / 100.;
        //dbg!(vel);

        if param.teclas.w {
            self.camera.move_forward(vel);
        }
        if param.teclas.s {
            self.camera.move_back(vel);
        }
        if param.teclas.d {
            self.camera.move_right(vel);
        }
        if param.teclas.a {
            self.camera.move_left(vel);
        }
        if param.teclas.q {
            self.camera.move_up(vel);
        }
        if param.teclas.e {
            self.camera.move_down(vel);
        }
    }
}