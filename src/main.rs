// https://www.youtube.com/watch?v=Ab8TOSFfNp4
// 12:34

extern crate gl;
extern crate sdl2;

use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use crate::player::Player;
use crate::scene::Scene;
use crate::settings::Parametros;

use crate::shader_program::ShaderProgram;
use crate::textures::Textures;

mod settings;
mod shader_program;
mod scene;
mod meshes;
mod camera;
mod player;
mod world_objects;
mod textures;

pub struct VoxelEngine {
    //sdl: Sdl,
    //events: Receiver<(f64, glfw::WindowEvent)>,
    //events: EventPump,
    clock: Instant,
    //delta_time: f64,
    last_time: f64,
    //time: f64,
    is_running: bool,
    nb_frames: u32,
    //window: sdl2::video::Window,
    //param: Parametros,
    shader_program: Option<ShaderProgram>,
    //vao: u32,
    scene: Option<Scene>,
    player: Option<Player>,
    //texture0: u32,
}

impl VoxelEngine {
    pub fn new(param: &mut Parametros) -> VoxelEngine {
        let is_running = true;
        let clock = Instant::now();

        //let mut textures = Textures::new();
        //let texture0 = textures.load("src/assets/test.png").unwrap();

        //dbg!(&clock);
        let delta_time = 0.;
        let time = 0.;
        // let is_running = true;
        let nb_frames = 0;
        let last_time = 0.;
        let player = None;

        // TODO Aqui quitamos la visibilidad del ratón

        let mut app = VoxelEngine {
            //param,
            shader_program: None,
            is_running,
            scene: None,
            nb_frames,
            last_time,
            clock,
            player,
            //texture0,
        };
        app.on_init(param);
        app
    }
    fn on_init(&mut self, param: &mut Parametros) {
        self.player = Some(Player::new(param));
        self.shader_program = Some(ShaderProgram::new(&self.player.as_ref().unwrap()));
        self.scene = Some(Scene::new(&self, param));
    }
    pub fn handle_events(&mut self, sdl: &Sdl, param: &mut Parametros) {
        let mut event_pump = sdl.event_pump().unwrap();
        'main: for event in event_pump.poll_iter() {
            // Manejar la entrada del usuario aquí
            match event {
                sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    //println!("MouseMotion");

                    //dbg!(xrel, yrel);
                    param.raton.x = x;
                    param.raton.y = y;

                    param.raton.xrel = xrel;
                    param.raton.yrel = yrel;
                }
                sdl2::event::Event::Quit { .. } => {
                    self.is_running = false;
                    break 'main;
                } // Rompe loop interno

                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::A, _) => {
                        //dbg!("A down");
                        param.teclas.a = true;
                    }
                    (Keycode::D, _) => {
                        //dbg!("D down");
                        param.teclas.d = true;
                    }
                    (Keycode::Q, _) => {
                        //dbg!("Q down");
                        param.teclas.q = true;
                    }
                    (Keycode::E, _) => {
                        //dbg!("E down");
                        param.teclas.e = true;
                    }
                    (Keycode::S, _) => {
                        //dbg!("S down");
                        param.teclas.s = true;
                    }
                    (Keycode::W, _) => {
                        //dbg!("W down");
                        param.teclas.w = true;
                    }
                    _ => (),
                }

                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => match (keycode, keymod) {
                    (Keycode::A, _) => {
                        //dbg!("A up");
                        param.teclas.a = false;
                    }
                    (Keycode::D, _) => {
                        //dbg!("D up");
                        param.teclas.d = false;
                    }
                    (Keycode::Q, _) => {
                        //dbg!("Q up");
                        param.teclas.q = false;
                    }
                    (Keycode::E, _) => {
                        //dbg!("E up");
                        param.teclas.e = false;
                    }
                    (Keycode::S, _) => {
                        //dbg!("S up");
                        param.teclas.s = false;
                    }
                    (Keycode::W, _) => {
                        //dbg!("W up");
                        param.teclas.w = false;
                    }
                    _ => (),
                }

                _ => {}
            }
        }
    }

    pub fn update(&mut self, param: &mut Parametros) {
        // LLama update de player
        self.player.as_mut().unwrap().update(self.clock, param);

        // LLama update de shader_prrogram
        let chunk = self.shader_program.as_mut().unwrap().chunk_id;
        self.shader_program.as_mut().unwrap().update(
            &self.player.as_mut().unwrap(), chunk);

        unsafe {
            // Da la orden de uso de programa shader
            gl::UseProgram(chunk);
            // Incrementa el contador de frames
            self.nb_frames += 1;

            let un_segundo = Duration::from_secs(1);
            if self.clock.elapsed() >= un_segundo { // Si la última salida es mayor de 1 segundo
                // TODO debe salir en la ventana
                //dbg!(self.nb_frames); // Presenta numero de frames y pon contador a cero
                self.nb_frames = 0;
                self.clock = Instant::now();
            }
        }
    }

    pub fn render(&mut self, param: &mut Parametros) {
        let r = param.bg_color.r;
        let g = param.bg_color.g;
        let b = param.bg_color.b;
        let a = param.bg_color.a;

        unsafe {
            // Crea un Viewport en la ventana y pone un color de fondo
            gl::Viewport(0, 0, param.win_res.x as i32, param.win_res.y as i32);

            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST); // Sino salen huecos
            //gl::Disable(gl::CULL_FACE); // Solo para probar
            //gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.scene.as_mut().unwrap().render(param);
    }
    pub fn run(&mut self, sdl: &Sdl, window: sdl2::video::Window, param: &mut Parametros) {
        while self.is_running {

            // Bucle de eventos
            self.handle_events(sdl, param);
            self.update(param);
            self.render(param);

            // Actualizar una ventana con renderizado OpenGL.
            window.gl_swap_window();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut param = Parametros::new();
    let sdl = sdl2::init().unwrap(); // Inicializa sdl2 devuelve Result<Sdl, String>

    // video devuelve Result<VideoSubsystem, String>
    let video_subsystem = sdl.video().unwrap();
    // gl_attr devuelve una estructura GLAttr
    let gl_attr = video_subsystem.gl_attr();

    // set_context_profile establece el Core profile
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5); // Establece version 4.5

    let window = video_subsystem  // Se ha creado constructor de Window
        .window("Game", param.win_res.x, param.win_res.y) // título, ancho, alto y devuelve un WindowBuilder
        .opengl() // flag SDL_WINDOW_OPENGL, ventana  con un contexto OpenGL y devuelve un WindowBuilder
        .resizable() // establece el flag SDL_WINDOW_RESIZABLE a true y devuelve un WindowBuilder
        .build()   // Construye Window Devuelve un Result<Window, WindowBuildError>
        .unwrap(); // Maneja el posible error

    // gl_create_context devuelve Result<GLContext, String>
    let _gl_context = window.gl_create_context().unwrap();

    // Una función para cargar el puntero de una función OpenGL con un string
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Imprimir la versión de OpenGL
    let version = unsafe {
        let data = gl::GetString(gl::VERSION) as *const i8;
        let version = std::ffi::CStr::from_ptr(data).to_bytes();
        String::from_utf8_lossy(version).into_owned()
    };

    println!("Versión de OpenGL: {}", version);

    let mut app = VoxelEngine::new(&mut param);

    app.run(&sdl, window, &mut param);
}