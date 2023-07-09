use nalgebra_glm::*;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Resolucion {
    pub x: u32,
    pub y: u32,
}

#[derive(Default, Debug)]
pub struct Teclas {
    pub q: bool,
    pub e: bool,
    pub a: bool,
    pub d: bool,
    pub s: bool,
    pub w: bool,
}

#[derive(Default, Debug)]
pub struct Raton {
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,

}

pub struct Parametros {
    pub win_res: Resolucion,
    pub titulo_ventana: String,
    pub bg_color: Color,
    pub aspect_ratio: f32,
    pub fov_deg: f32,
    pub v_fov: f32,
    pub h_fov: f32,
    pub near: f32,
    pub far: f32,
    pub pitch_max: f32,
    pub player_speed: f32,
    pub player_rot_speed: f32,
    pub player_pos: TVec3<f32>,
    pub mouse_sensivity: f32,
    pub teclas: Teclas,
    pub raton: Raton,
    pub chunk_size: f32,
    pub h_chunk_size: f32,
    pub chunk_area: f32,
    pub chunk_vol: f32,
}

impl Parametros {
    pub fn new() -> Parametros {
        let x = 1200;
        let y = 1200;
        let win_res = Resolucion { x, y };
        let titulo_ventana = ("Ventanita").to_string();
        // Chunck
        let chunk_size: f32 = 32.;
        let h_chunk_size = chunk_size / 2.;
        let chunk_area = chunk_size * chunk_size;
        let chunk_vol = chunk_area * chunk_size;
        // Camara
        let aspect_ratio: f32 = x as f32 / y as f32;
        let fov_deg: f32 = 50.;
        let v_fov: f32 = (fov_deg).to_radians();// fov vertical
        let h_fov: f32 = 2. * ((v_fov * 0.5).tan() * aspect_ratio).atan(); // fov horizontal
        let near: f32 = 0.1;
        let far: f32 = 2000.0;
        let pitch_max: f32 = (89.0f32).to_radians();

        // Player
        //let player_speed = 0.005;
        let player_speed = 0.05;
        //let player_rot_speed = 0.003;
        let player_rot_speed = 6.;
        let player_pos = TVec3::new(h_chunk_size, chunk_size, 1.5 * chunk_size);
        //let player_pos = TVec3::new(1000., 200., 1440.);
        // Entradas
        //let mouse_sensivity = 0.0002;
        let mouse_sensivity = 0.0008;
        let teclas = Teclas::default();
        let raton = Raton::default();
        // Colores
        let bg_color = Color { r: 0.1, g: 0.16, b: 0.25, a: 1.0 };

        Parametros {
            win_res,
            titulo_ventana,
            bg_color,
            aspect_ratio,
            fov_deg,
            v_fov,
            h_fov,
            near,
            far,
            pitch_max,
            player_speed,
            player_rot_speed,
            player_pos,
            mouse_sensivity,
            teclas,
            raton,
            chunk_size,
            h_chunk_size,
            chunk_area,
            chunk_vol,
        }
    }
}
