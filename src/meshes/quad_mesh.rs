use crate::meshes::base_mesh::BaseMesh;
use crate::settings::Parametros;
use crate::VoxelEngine;

pub struct QuadMesh {
    //app: VoxelEngine, si necesitamos app, enviarlo en otros métodos
    pub base_mesh: BaseMesh,

    // Programa Shader
    program: u32,

    // Por ahora no se usa. Nombres de atributos según el formato: ("in_position,"in_color")
    attrs: Vec<String>,

    // VAO
    vao: Option<u32>,

    // Datos del buffer de vertices, formato: "3f 3f"
    vbo_format: String,

    // en este caso al ser u8 es 1
    format_size: usize,

}

impl QuadMesh {
    pub fn new(app: &VoxelEngine) -> QuadMesh {
        let mut base_mesh = BaseMesh::new();
        let program = app.shader_program.as_ref().unwrap().quad_id;
        let vbo_format = "3f,3f".to_string();
        let attrs = vec!["in_position".to_string(), "in_color".to_string()];
        let format_size = 5; // x,y,z y 2 más que no se lo que es

        let mut qm = QuadMesh {
            //app,
            base_mesh,
            program,
            attrs,
            vao: None,
            vbo_format,
            format_size,
        };

        qm.vao = qm.get_vao();
        qm
    }

    pub fn get_vertex_data() -> Vec<i32> {
        let vertices: Vec<i32> = vec![
            // posiciones      // colores
            0, 0, 0, 0, 0,
            1, 0, 1, 1, 1,
            1, 0, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 1, 0,
            1, 0, 1, 0, 1,
        ];

        //vertex_data = np.hstack([tex_coords, vertices])
        vertices
    }

    pub fn get_len_vertex_data() -> usize {
        30
    }

    // Funcion intermedia puesta por mi

    pub fn get_vao(&mut self) -> Option<u32> {
        let vertex_data: Vec<i32> = QuadMesh::get_vertex_data();
        self.base_mesh.get_vao(vertex_data)
    }

    // Funcion intermedia puesta por mi
    pub fn render(&mut self, param: &mut Parametros) {
        self.base_mesh.render(self.vao, QuadMesh::get_len_vertex_data(), param);
    }
}
