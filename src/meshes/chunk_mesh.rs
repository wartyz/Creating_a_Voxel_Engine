use crate::meshes::base_mesh::BaseMesh;
use crate::meshes::chunk_mesh_builder::build_chunk_mesh;
use crate::settings::Parametros;
use crate::VoxelEngine;
use crate::world_objects::chunk::Chunk;

pub struct ChunkMesh {
    //app: VoxelEngine, si necesitamos app, enviarlo en otros métodos
    // Chunkmesh tiene un chunk, no al reves
    chunk: Chunk,

    // Seudo herencia
    pub base_mesh: BaseMesh,

    // Programa Shader
    program: u32,

    // Por ahora no se usa. Nombres de atributos según el formato: ("in_position,"voxel_id", "face_ide")
    attrs: Vec<String>,

    // VAO
    vao: Option<u32>,

    // Datos del buffer de vertices, formato: "3u1 1u1 1u1"
    vbo_format: String,

    // en este caso al ser u8 es 1
    format_size: usize,

    // Tamaño del vertex data
    len_vertex_data: usize,
}

//
impl ChunkMesh {
    pub fn new(app: &VoxelEngine, param: &mut Parametros) -> ChunkMesh {
        let chunk = Chunk::new(param);
        let mut base_mesh = BaseMesh::new();
        let program = app.shader_program.as_ref().unwrap().chunk_id;

        let vbo_format = "3u1 1u1 1u1".to_string();
//         let format_size = 0;
        let attrs = vec!["in_position".to_string(), "voxel_id".to_string(), "face_ide".to_string()];
//         let vao = base_mesh.get_vao();
        let format_size = 5; // son 5 bytes
        let len_vertex_data = 0;
        let mut cm = ChunkMesh {
            chunk,
            base_mesh,
            program,
            vao: None,
            vbo_format,
            format_size,
            attrs,
            len_vertex_data,
        };
        cm.get_vao(param);
        cm
    }

    // Al no ponerla en Chunk no se si seria conveniente ponerla aquí
//     pub fn build_mesh(&mut self, app: &VoxelEngine) {
//         self.mesh = Some(ChunckMesh::new(app));
//     }
    // Al no ponerla en Chunk no se si seria conveniente ponerla aquí
//     pub fn render(&mut self) {
//         self.mesh.as_mut().unwrap().render();
//     }
//

    // Funcion intermedia puesta por mi
    pub fn get_vao(&mut self, param: &mut Parametros) {
        let vertex_data: Vec<i32> = self.get_vertex_data(param);
        self.len_vertex_data = vertex_data.len();

        self.vao = self.base_mesh.get_vao(vertex_data);
        dbg!(self.vao);
    }

    // recibe &chunk en vez de tener &chunk en la struct
    pub fn get_vertex_data(&mut self, param: &mut Parametros) -> Vec<i32> {
        // build_chunk_mesh está en chunk_mesh_builder

        let mesh = build_chunk_mesh(
            &self.chunk.voxels, // hay 32 * 32 * 32 voxels
            self.format_size, // 5 que son los 5 atributos)
            param,
        );
        mesh
    }

    //
    pub fn render(&mut self, param: &mut Parametros) {
        self.base_mesh.render(self.vao, self.len_vertex_data, param);
    }
}