use crate::settings::Parametros;

// Comprueba si el voxel es espacio (vacio)
pub fn is_void(voxel_pos: (i32, i32, i32), chunck_voxels: &Vec<u8>, param: &mut Parametros) -> bool {
    let (x, y, z) = voxel_pos;
    if x == -1 || y == -1 || z == -1 {
        return true;
    }
    let chunk_size = param.chunk_size as i32;
    let chunk_area = param.chunk_area;
    if (0 <= x && x < chunk_size) && (0 <= y && y < chunk_size) && (0 <= z && z < chunk_size) {
        // chunk_voxels tiene 32768 elementos
        if chunck_voxels[x as usize + chunk_size as usize * z as usize + chunk_area as usize * y as usize] != 0 {
            return false;
        }
    }

    true
}

// vertex_data se irá rellenando a medida que reciba VertexAttr en vertices
pub fn add_data(vertex_data: &mut Vec<i32>, mut index: usize, vertices: &Vec<&Vec<i32>>) -> usize {
    //dbg!(index);
    //dbg!(vertices.len());
    //dbg!(vertices);

    for vertex in vertices {
        //println!("en add_data()/chunk_mesh_builder");
        for n in 0..5 {
            //dbg!( n, vertex[n]);
            vertex_data[index] = vertex[n];
            index += 1;
        }
        //println!("");
    }
    index
}

pub fn build_chunk_mesh(chunk_voxels: &Vec<u8>, format_size: usize, param: &mut Parametros) -> Vec<i32> {
    // Maximo 3 caras visibles * 4 puntos  = 12, peo como en vertex_data almacenamos vertex duplicados
    // serán 3 caras visibles * 6 puntos = 18 puntos
    // y como cada punto tiene 5 atibutos (format_size)-> * 5


    let mut vertex_data: Vec<i32> = vec![0; param.chunk_vol as usize * 18 * format_size];
    //let mut vertex_data: Vec<u8> = Vec::with_capacity(param.chunk_vol as usize * 18 * format_size * 5);
    //dbg!(vertex_data.len());
    let mut index = 0;

    let chunk_size = param.chunk_size as usize;
    let chunk_area = param.chunk_area as usize;
    for x in 0..chunk_size as i32 {
        for y in 0..chunk_size as i32 {
            for z in 0..chunk_size as i32 {

                // hay 32768 voxels * 60 u8 = 1966080

                // chunk_voxels tiene 32768 elementos
                //dbg!(x as usize + chunk_size * z as usize + chunk_area * y as usize);
                let voxel_id = chunk_voxels[x as usize + chunk_size * z as usize + chunk_area * y as usize] as i32;

                if voxel_id == 0 {
                    continue; // es "aire"
                }
                // los atributos son -> x, y, z, voxel_id, face_id
                // Cara superior, si está vacio renderizar
                if is_void((x, y + 1, z), chunk_voxels, param) {
                    let v0 = vec![x, y + 1, z, voxel_id, 0];
                    let v1 = vec![x + 1, y + 1, z, voxel_id, 0];
                    let v2 = vec![x + 1, y + 1, z + 1, voxel_id, 0];
                    let v3 = vec![x, y + 1, z + 1, voxel_id, 0];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v3, &v2, &v0, &v2, &v1])
                }
                // Cara inferior, si está vacio renderizar
                if is_void((x, y - 1, z), chunk_voxels, param) {
                    let v0 = vec![x, y, z, voxel_id, 1];
                    let v1 = vec![x + 1, y, z, voxel_id, 1];
                    let v2 = vec![x + 1, y, z + 1, voxel_id, 1];
                    let v3 = vec![x, y, z + 1, voxel_id, 1];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v2, &v3, &v0, &v1, &v2])
                }
                // Cara derecha, si está vacio renderizar
                if is_void((x + 1, y, z), chunk_voxels, param) {
                    let v0 = vec![x + 1, y, z, voxel_id, 2];
                    let v1 = vec![x + 1, y + 1, z, voxel_id, 2];
                    let v2 = vec![x + 1, y + 1, z + 1, voxel_id, 2];
                    let v3 = vec![x + 1, y, z + 1, voxel_id, 2];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v1, &v2, &v0, &v2, &v3])
                }
                // Cara izquierda, si está vacio renderizar
                if is_void((x - 1, y, z), chunk_voxels, param) {
                    let v0 = vec![x, y, z, voxel_id, 3];
                    let v1 = vec![x, y + 1, z, voxel_id, 3];
                    let v2 = vec![x, y + 1, z + 1, voxel_id, 3];
                    let v3 = vec![x, y, z + 1, voxel_id, 3];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v2, &v1, &v0, &v3, &v2])
                }
                // Cara posterior, si está vacio renderizar
                if is_void((x, y, z - 1), chunk_voxels, param) {
                    let v0 = vec![x, y, z, voxel_id, 4];
                    let v1 = vec![x, y + 1, z, voxel_id, 4];
                    let v2 = vec![x + 1, y + 1, z, voxel_id, 4];
                    let v3 = vec![x + 1, y, z, voxel_id, 4];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v1, &v2, &v0, &v2, &v3])
                }
                // Cara frontal, si está vacio renderizar
                if is_void((x, y, z + 1), chunk_voxels, param) {
                    let v0 = vec![x, y, z + 1, voxel_id, 5];
                    let v1 = vec![x, y + 1, z + 1, voxel_id, 5];
                    let v2 = vec![x + 1, y + 1, z + 1, voxel_id, 5];
                    let v3 = vec![x + 1, y, z + 1, voxel_id, 5];

                    index = add_data(&mut vertex_data, index, &vec![&v0, &v2, &v1, &v0, &v3, &v2])
                }
            }
        }
    }

    //vertex_data
    dbg!(vertex_data.len());
    //let kk = vertex_data.split_off(index);

    // for n in 0..vertex_data.len() {
    //     dbg!(n,&vertex_data[n]);
    // }

    // pbas:
    let subset_vec: Vec<i32> = vertex_data[0..index].to_vec();

    dbg!(vertex_data.len());
    dbg!(subset_vec.len());
    //vertex_data

    subset_vec
}