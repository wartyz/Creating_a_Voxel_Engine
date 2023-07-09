// use crate::meshes::chunk_mesh::ChunckMesh;
use crate::settings::Parametros;
use crate::VoxelEngine;
use noise::NoiseFn;

pub struct Chunk {
    pub voxels: Vec<u8>,
//     no tiene un ChunkMesh, sino ChunkMesh tiene un Chunk
}

impl Chunk {
    pub fn new(param: &mut Parametros) -> Chunk {
        let voxels = Chunk::build_voxels(param);
        dbg!(voxels.len());

        let mut chunk = Chunk {
            voxels,
//             mesh: None,
        };
//         chunk.build_mesh(app);
        chunk
    }

    pub fn build_mesh(&mut self, app: &VoxelEngine) {
        // No lo pongo seria una funcion recursiva
//         self.mesh = Some(ChunckMesh::new(app));
    }

    pub fn render(&mut self) {
        // No lo pongo seria una funcion recursiva
//         self.mesh.as_mut().unwrap().render();
    }

    pub fn build_voxels(param: &mut Parametros) -> Vec<u8> {
        // Chunk vacio, 0 significa espacio vacio, hay 32 * 32 * 32 elementos de valor u8 0->254

        let mut voxels = vec![0u8; param.chunk_vol as usize];

        let chunck_size = param.chunk_size as usize;
        let chunck_area = param.chunk_area as usize;

        for x in 0..chunck_size {
            for z in 0..chunck_size {
                for y in 0..chunck_size {
                    // Le damos a todos los voxels un valor 1
                    //voxels[x + chunck_size * z + chunck_area * y] = (x + y + z) as u8;
                    let perlin = noise::Perlin::new(1);
                    let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1]);
                    //dbg!(val);
                    if val < 0. {
                        voxels[x + chunck_size * z + chunck_area * y] = (x + y + z) as u8;
                    } else {
                        voxels[x + chunck_size * z + chunck_area * y] = 0;
                    }
                }
            }
        }
        //dbg!(&voxels);
        voxels
    }
}