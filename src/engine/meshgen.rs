use cgmath::Vector3;

use crate::{engine::{data::{BlockId, CHUNK_SIZE, Vertex}, mods::blocks::BlockRegistry}, game::chunk::manager::ChunkNeighborhood};

struct Face {
    pub normal: [i8; 3],
    pub verts: [[u32; 3]; 4],
    pub uv_indices: [u32; 4]
}

const FACES: [Face; 6] = [
    // +z (front)
    Face { 
        normal: [0, 0, 1], 
        verts: [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]], 
        uv_indices: [2, 3, 0, 1]
    },
    // -z (back)
    Face { 
        normal: [0, 0, -1], 
        verts: [[1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 1, 0]], 
        uv_indices: [2, 3, 0, 1]
    },
    // +x (right)
    Face { 
        normal: [1, 0, 0], 
        verts: [[1, 0, 1], [1, 0, 0], [1, 1, 0], [1, 1, 1]], 
        uv_indices: [2, 3, 0, 1]
    },
    // -x (left)
    Face { 
        normal: [-1, 0, 0], 
        verts: [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]], 
        uv_indices: [2, 3, 0, 1]
    },
    // +y (top)
    Face { 
        normal: [0, 1, 0], 
        verts: [[0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0]], 
        uv_indices: [0, 1, 2, 3] 
    },
    // -y (bottom)
    Face { 
        normal: [0, -1, 0], 
        verts: [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]], 
        uv_indices: [0, 1, 2, 3] 
    },
];

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub world_pos: Vector3<i32>,
}

pub fn mesh_chunk(neighborhood: &ChunkNeighborhood, block_registry: &BlockRegistry) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices:  Vec<u32>    = Vec::new();
    let chunk = &neighborhood.center;
    let s = CHUNK_SIZE as i32;

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let block = chunk.get(x, y, z);
                if block == BlockId::AIR { continue; }

                for (i, face) in FACES.iter().enumerate() {
                    let nx = x as i32 + face.normal[0] as i32;
                    let ny = y as i32 + face.normal[1] as i32;
                    let nz = z as i32 + face.normal[2] as i32;

                    let neighbor_is_air = if nx < 0 || ny < 0 || nz < 0 || nx >= s || ny >= s || nz >= s {
                        if let Some(neighbor) = &neighborhood.neighbors[i] {
                            let nnx = if nx < 0 { s - 1 } else if nx >= s { 0 } else { nx } as usize;
                            let nny = if ny < 0 { s - 1 } else if ny >= s { 0 } else { ny } as usize;
                            let nnz = if nz < 0 { s - 1 } else if nz >= s { 0 } else { nz } as usize;
                            
                            neighbor.get(nnx, nny, nnz) == BlockId::AIR
                        } else {
                            true
                        }
                    } else {
                        chunk.get(nx as usize, ny as usize, nz as usize) == BlockId::AIR
                    };

                    if neighbor_is_air {
                        let base_index = vertices.len() as u32;
                        for (v_idx, v) in face.verts.iter().enumerate() {
                            let uv_id = face.uv_indices[v_idx];
                            let def = &block_registry.definitions[block.0 as usize];

                            let tex_id = match face.normal {
                                [0, 1, 0]  => def.textures.top,
                                [0, -1, 0] => def.textures.bottom,
                                _          => def.textures.sides, // all 4 sides use this
                            };
                            
                            let packed: u32 = crate::engine::data::pack_information(
                                x as u32 + v[0],
                                y as u32 + v[1],
                                z as u32 + v[2],
                                face.normal,
                                uv_id
                            );
                            
                            vertices.push(Vertex { packed, packed2: tex_id as u32 });
                        }
                        indices.extend_from_slice(&[
                            base_index, base_index + 1, base_index + 2,
                            base_index, base_index + 2, base_index + 3
                        ]);
                    }
                }
            }
        }
    }

    Mesh { 
        vertices, 
        indices, 
        world_pos: Vector3::new(
            chunk.position.x * CHUNK_SIZE as i32, 
            chunk.position.y * CHUNK_SIZE as i32, 
            chunk.position.z * CHUNK_SIZE as i32
        )
    }
}