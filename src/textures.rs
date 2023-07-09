use std::path::Path;
use gl::types::*;
use sdl2::libc::open;
use image::*;

// https://github.com/Bastczuak/sdl2-opengl/blob/master/src/main.rs
pub struct Textures {}

impl Textures {
    pub fn new() -> Textures {
        Textures {}
    }

    pub fn load(&mut self, file_name: &str) -> Result<u32, String> {
        let img = image::open(&Path::new(file_name)).map_err(|e| format!("No se lee textura{}", e)).unwrap();
        let (width, height) = img.dimensions();

        dbg!(width);
        dbg!(height);

        let format = match img.color() {
            ColorType::Rgb8 => gl::RGB,
            ColorType::Rgba8 => gl::RGBA,
            _ => {
                println!("No se detecta Colortype de textura");
                todo!();
            }
            // ColorType::Gray(_) => gl::DEPTH_COMPONENT,
            // ColorType::GrayA(_) => gl::DEPTH_STENCIL,
            // ColorType::BGR(_) => gl::BGR,
            // ColorType::BGRA(_) => gl::BGRA,
            // ColorType::Palette(_) => gl::DEPTH_COMPONENT,
        };

        // match img.color() {
        //     ColorType::Rgb8 => println!("Rgb8"),
        //     ColorType::Rgba8 => println!("Rgb8"),
        //     _ => println!("Ninguno"),
        // };

        unsafe {
            // initialize texture
            let mut texture_id = 0;
            gl::GenTextures(1, &mut texture_id);
           
            // set the texture wrapping parameters
            // set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            // transfiere datos de imagen
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                img.width() as GLint,
                img.height() as GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const GLvoid,
            );

            // generate all mip map images for us
            // self.textures.push(texture);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Desconectamos textura
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Ok(texture_id)
        }
    }
}