use gl::types::GLsizei;
use crate::settings::Parametros;

pub struct BaseMesh {}

impl BaseMesh {
    pub fn new() -> BaseMesh {
        BaseMesh {}
    }
    // pub fn get_vertex_data(&mut self) -> Vec<f32> {
    //     let vertices: Vec<f32> = vec![
    //         // posiciones      // colores
    //         0.5, 0.5, 0.0, 0., 1., 0.,
    //         -0.5, 0.5, 0.0, 1., 0., 1.,
    //         -0.5, -0.5, 0.0, 1., 1., 0.,
    //         0.5, 0.5, 0.0, 0., 1., 0.,
    //         -0.5, -0.5, 0.0, 1., 1., 0.,
    //         0.5, -0.5, 0.0, 0., 0., 1.,
    //     ];
    //
    //     //vertex_data = np.hstack([tex_coords, vertices])
    //     vertices
    // }

    pub fn get_vao(&mut self, vertex_data: Vec<i32>) -> Option<u32> {
        //dbg!(&vertex_data);
        //let vertex_data = self.get_vertex_data();

        let mut vao_id: u32 = 0;
        unsafe {
            // Crea VBO
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            // Ahora cargamos datos en el buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER, // objetivo
                // Tamaño de los datos en bytes tomando el tamaño de un f32 con std::mem::size_of::<f32>(),
                // multiplicándolo por el número de elementos en el vector (vertices.len()),
                // y luego forzándolo al entero gl::types::GLsizeiptr
                (vertex_data.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                // Obtenemos un puntero vacío a los datos mediante la función .as_ptr(),
                // pero nos devuelve *const f32, mientras que OpenGL necesita *const GLvoid
                vertex_data.as_ptr() as *const gl::types::GLvoid, // puntero a datos
                gl::STATIC_DRAW,                               // uso
            );

            // Crea VAO
            gl::GenVertexArrays(1, &mut vao_id);

            //Activa el VAO
            gl::BindVertexArray(vao_id);// Vinculamos el VAO

            //gl::BindBuffer(gl::ARRAY_BUFFER, 0); // desenlazar el buffer

            //gl::GenVertexArrays(1, &mut vao_id);

            //gl::BindVertexArray(vao_id);
            //gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Vinculamos el VBO

            gl::EnableVertexAttribArray(0); // esto es "layout (location = 0)" en el vertex shader
            gl::VertexAttribPointer(
                0,         // índice del atributo de vértices genérico ("layout (location = 0)")
                3,          // número de componentes del atributo de vértices genérico
                gl::INT, // tipo de datos, ahora es u8
                gl::FALSE, // normalizado (conversión int-a-float )
                // stride (separación en bytes entre attributos consecutivos)
                (5 * std::mem::size_of::<i32>()) as gl::types::GLint,
                std::ptr::null(), // desplazamiento del primer componente
            );

            // voxel_id
            gl::EnableVertexAttribArray(1); // esto es "layout (location = 1)" en el vertex shader
            gl::VertexAttribPointer(
                1,         // índice del atributo de vértices genérico ("layout (location = 1)")
                1,          // número de componentes del atributo de vértices genérico (voxel_id)
                gl::INT, // tipo de datos ¿porqué va bien FLOAT?
                gl::FALSE, // normalizado (conversión int-a-float )
                // stride (separación en bytes entre attributos consecutivos)
                (5 * std::mem::size_of::<i32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<i32>()) as *const gl::types::GLvoid, // offset del primer componente
            );

            // face_id
            gl::EnableVertexAttribArray(2); // esto es "layout (location = 2)" en el vertex shader
            gl::VertexAttribPointer(
                2,         // índice del atributo de vértices genérico ("layout (location = 2)")
                1,          // número de componentes del atributo de vértices genérico (face_id)
                gl::INT, // tipo de datos
                gl::FALSE, // normalizado (conversión int-a-float )
                // stride (separación en bytes entre attributos consecutivos)
                (5 * std::mem::size_of::<i32>()) as gl::types::GLint,
                (4 * std::mem::size_of::<i32>()) as *const gl::types::GLvoid, // offset del primer componente
            );

            // desvinculamos VBO y VAO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // set up shared state for window
            //gl::Viewport(0, 0, param.win_res.x as i32, param.win_res.y as i32); // configurar el viewport
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);

            Some(vao_id)
        }
    }

    pub fn render(&mut self, vao: Option<u32>, len_vertex_data: usize, param: &mut Parametros) {
        unsafe {
            //dbg!(model.get_vertex_count());
            //dbg!(model.get_vao_id());
            gl::BindVertexArray(vao.unwrap());
            //gl::EnableVertexAttribArray(0);
            //gl::EnableVertexAttribArray(1);

            gl::DrawArrays(
                gl::TRIANGLES, // modo
                0,             // índice inicial en el arreglo habilitado
                //param.chunk_vol as GLsizei);//model.get_vertex_count()); // número de índices a renderizar
                len_vertex_data as GLsizei); //model.get_vertex_count()); // número de índices a renderizar

            //dbg!("vertex count=",model.get_vertex_count());
            //gl::DisableVertexAttribArray(1);
            //gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }
}