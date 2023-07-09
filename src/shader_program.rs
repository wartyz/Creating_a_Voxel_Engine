use std::fs::File;
use std::io::Read;
use std::ptr;
use crate::player::Player;

use std::ffi::{CStr, CString};
use crate::textures::Textures;

pub struct ShaderProgram {
    pub quad_id: u32,
    pub chunk_id: u32,
    //pub texture0: u32,
}

impl ShaderProgram {
    pub fn new(player: &Player) -> ShaderProgram {
        // No tengo claro si debe estar aqui o en main


        let quad_id = ShaderProgram::get_program("quad");
        let chunk_id = ShaderProgram::get_program("chunk");

        ShaderProgram::set_uniforms_on_init(player, quad_id);
        ShaderProgram::set_uniforms_on_init(player, chunk_id);

        ShaderProgram {
            quad_id,
            chunk_id,
            //texture0,
        }
    }

    pub fn set_uniforms_on_init(player: &Player, program_id: u32) {
        let mut textures = Textures::new();
        //let texture_id0 = textures.load("src/assets/test.png").unwrap();

        let texture_id0 =
            textures.load("/media/luis/5nkg/Proyectos_Programacion/Rust/Coder_Space \
            (YT)/Creating_a_Voxel_SDL2K/main/src/assets/frame.png").unwrap();

        dbg!(texture_id0);

        let model_matrix = nalgebra_glm::TMat4::identity();
        let proj_matrix = player.camera.m_proj;
        // let view_matrix = player.camera.m_view;

        unsafe {
            let textu0 = CString::new("u_texture_0").unwrap();
            let location_texture_0 = gl::GetUniformLocation(
                program_id,
                textu0.as_ptr(),
            );

            let nombre1 = CString::new("m_model").unwrap();
            let location_model_matrix = gl::GetUniformLocation(
                program_id,
                nombre1.as_ptr(),
            );

            let nombre3 = CString::new("m_proj").unwrap();
            let location_proj_matrix = gl::GetUniformLocation(
                program_id,
                nombre3.as_ptr(),
            );
            dbg!(location_texture_0);
            dbg!(location_model_matrix);
            dbg!(location_proj_matrix);

            //let mut texture_id: gl::types::GLuint = 0;
            //gl::GenTextures(1, &mut texture_id);
            gl::UseProgram(program_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id0);
            gl::Uniform1i(location_texture_0, 0);

            gl::UniformMatrix4fv(
                location_model_matrix,
                1,
                gl::FALSE,
                model_matrix.as_ptr(),
            );

            gl::UniformMatrix4fv(
                location_proj_matrix,
                1,
                gl::FALSE,
                proj_matrix.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, player: &Player, chunk: u32) {
        let view_matrix = player.camera.m_view;

        let nombre2 = CString::new("m_view").unwrap();
        unsafe {
            let location_view_matrix = gl::GetUniformLocation(
                chunk,
                nombre2.as_ptr(),
            );

            //dbg!(location_view_matrix);
            //dbg!(*view_matrix.as_ptr());

            // no se si hay que llamar a  gl::UseProgram(quad);

            gl::UniformMatrix4fv(
                location_view_matrix,
                1,
                gl::FALSE,
                view_matrix.as_ptr(),
            );
        }
    }

    pub fn get_program(shader_name: &str) -> u32 {
        // Vertex Shader *************************
        //let fv = format!("src/shaders/{}.vert", shader_name);
        let fv = format!("/media/luis/5nkg/Proyectos_Programacion/Rust/Coder_Space (YT)/Creating_a_Voxel_SDL2K/main/src/shaders/{}.vert", shader_name);
        let mut vertex_file = match File::open(&fv) {
            Err(error) => panic!("no se puede abrir {:?}: {}", fv, error),
            Ok(f) => f,
        };
        let mut file_data_vs = String::new();
        let _ = vertex_file.read_to_string(&mut file_data_vs);

        // Fragment Shader *************************
        //let ff = format!("src/shaders/{}.frag", shader_name);
        let ff = format!("/media/luis/5nkg/Proyectos_Programacion/Rust/Coder_Space (YT)/Creating_a_Voxel_SDL2K/main/src/shaders/{}.frag", shader_name);
        //let ff = format!("/home/luis/pba/shaders/{}.frag", shader_name);
        let mut frag_file = match File::open(&ff) {
            Err(error) => panic!("no se puede abrir {:?}: {}", ff, error),
            Ok(f) => f,
        };
        let mut file_data_fs = String::new();
        let _ = frag_file.read_to_string(&mut file_data_fs);

        unsafe {
            // Vertex Shader *************************
            let vs_id = gl::CreateShader(gl::VERTEX_SHADER); // crea shader
            let sh_c_str = CString::new(file_data_vs).unwrap();

            // carga lo que habia en fichero
            gl::ShaderSource(vs_id, 1, &sh_c_str.as_ptr(), ptr::null());
            gl::CompileShader(vs_id); // lo compila

            //Comprueba errores
            let mut status: i32 = 0;
            gl::GetShaderiv(vs_id, gl::COMPILE_STATUS, &mut status);
            if status == 0 {
                panic!("ERROR: No se puede compilar shader {:?}", fv);
            }

            // Fragment Shader *************************
            let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER); // crea shader
            let sh_c_str = CString::new(file_data_fs).unwrap();

            // carga lo que habia en fichero
            gl::ShaderSource(fs_id, 1, &sh_c_str.as_ptr(), ptr::null());
            gl::CompileShader(fs_id); // lo compila

            //Comprueba errores
            let mut status: i32 = 0;
            gl::GetShaderiv(vs_id, gl::COMPILE_STATUS, &mut status);
            if status == 0 {
                panic!("ERROR: No se puede compilar shader {:?}", ff);
            }

            // Creamos Programa shader mezcla de VS y FS

            let program_id = gl::CreateProgram();

            //Activamos shaders enlazandolos
            gl::AttachShader(program_id, vs_id);
            gl::AttachShader(program_id, fs_id);

            //ShaderProgram::bind_attributes();

            //linkamos programa shader
            gl::LinkProgram(program_id);
            gl::ValidateProgram(program_id);

            let mut status: i32 = 0;
            gl::GetProgramiv(program_id, gl::VALIDATE_STATUS, &mut status);
            if status == 0 {
                println!("ERROR: Programa Shader no validado");
            }
            program_id
        }

        // let vertex_shader: GLuint =
        //     ShaderProgram::load_shader(vertex_file, gl::VERTEX_SHADER);
        // let fragment_shader_id: GLuint =
        //     ShaderProgram::load_shader(frag_file, gl::FRAGMENT_SHADER);
    }
}