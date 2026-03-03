use crate::engine::{data::{BlockTypes, CHUNK_SIZE, Vertex}};
use crate::game::chunk::Chunk;

struct Face {
    pub normal: [i8; 3],
    pub verts: [[i32; 3]; 4],
}

const FACES: [Face; 6] = [
    // +z (front)
    Face { normal: [0, 0, 1], verts: [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]] },
    // -z (back)
    Face { normal: [0, 0, -1], verts: [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]] },
    // +x (right)
    Face { normal: [1, 0, 0], verts: [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]] },
    // -x (left)
    Face { normal: [-1, 0, 0], verts: [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]] },
    // +y (top)
    Face { normal: [0, 1, 0], verts: [[0, 1, 0], [0, 1, 1], [1, 1, 1], [1, 1, 0]] },
    // -y (bottom)
    Face { normal: [0, -1, 0], verts: [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]] },
];

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

// hello square!
impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: vec![
                Vertex { position: [-1, -1, 0], packed: 0 }, // bottom-left
                Vertex { position: [1 , -1, 0], packed: 0 }, // bottom-right
                Vertex { position: [1 , 1 , 0], packed: 0 }, // top-right
                Vertex { position: [-1, 1 , 0], packed: 0 }  // top-left
            ],
            indices: vec![0, 1, 2, 2, 3, 0]
        }
    }
}

fn pack_information(normal: [i8; 3], block_type: BlockTypes) -> u32 {
    let btype = block_type as u8;
    let dir = match normal { // todo: integrate this into FACES
        [1, 0, 0] => 0,
        [-1, 0, 0] => 1,
        [0, 1, 0] => 2,
        [0, -1, 0] => 3,
        [0, 0, 1] => 4,
        [0, 0, -1] => 5,
        _ => 0, // default case
    };
    (btype as u32) | ((dir as u32) << 8)
}

pub fn mesh_chunk(chunk: &Chunk) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices:  Vec<u32>    = Vec::new();

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let block = chunk.get(x, y, z);
                if block == BlockTypes::AIR {continue;};

                for face in FACES.iter() {
                    // get normals for neighbor checking
                    let nx = x as i32 + face.normal[0] as i32;
                    let ny = y as i32 + face.normal[1] as i32;
                    let nz = z as i32 + face.normal[2] as i32;

                    let neighbor_is_air = nx < 0 || ny < 0 || nz < 0 || 
                                         nx >= CHUNK_SIZE as i32 || ny >= CHUNK_SIZE as i32 || nz >= CHUNK_SIZE as i32 ||
                                         chunk.get(nx as usize, ny as usize, nz as usize) == BlockTypes::AIR;

                    if neighbor_is_air {
                        let base_index = vertices.len() as u32;
                        for v in face.verts {
                            let packed: u32 = pack_information(face.normal, block);
                            vertices.push(Vertex {
                                position: [
                                    (x as i32 + v[0]) + chunk.position.x as i32 * CHUNK_SIZE as i32,
                                    (y as i32 + v[1]) + chunk.position.y as i32 * CHUNK_SIZE as i32,
                                    (z as i32 + v[2]) + chunk.position.z as i32 * CHUNK_SIZE as i32,
                                ],
                                packed
                            });
                        }

                        indices.extend_from_slice(&[
                            base_index, base_index+1, base_index+2,
                            base_index, base_index+2, base_index+3
                        ]);
                    }
                }
            }
        }
    }
    
    Mesh { vertices, indices }
}