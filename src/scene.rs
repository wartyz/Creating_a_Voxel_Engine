use crate::meshes::chunk_mesh::ChunkMesh;
use crate::meshes::quad_mesh::QuadMesh;
use crate::settings::Parametros;
use crate::VoxelEngine;

pub struct Scene {
    //app: VoxelEngine,
    //quad_mesh: QuadMesh,
    chunk_mesh: ChunkMesh,

}

impl Scene {
    pub fn new(app: &VoxelEngine, param: &mut Parametros) -> Scene {
        //let quad_mesh = QuadMesh::new(app);
        let chunk_mesh = ChunkMesh::new(app, param);
        Scene {
            // app,
            //quad_mesh,
            chunk_mesh,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, param: &mut Parametros) {
        //self.quad_mesh.render();
        self.chunk_mesh.render(param);
    }
}