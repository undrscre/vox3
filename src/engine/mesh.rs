use crate::engine::{data::{Vertex, BlockTypes}, world::{World, get_idx, WORLD_SIZE}};

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

// note: this name is a lie at this point in development it's actually meshing the whole world
// todo: ^ get rid of this once you're actually meshing the chunk
pub fn mesh_chunk(chunk: &World) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices:  Vec<u32>    = Vec::new();

    for z in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let block = chunk.data[get_idx(x, y, z)];
                if block == BlockTypes::AIR {continue;};

                for face in FACES.iter() {
                    // get normals for neighbor checking
                    let nx = x as i32 + face.normal[0] as i32;
                    let ny = y as i32 + face.normal[1] as i32;
                    let nz = z as i32 + face.normal[2] as i32;

                    let neighbor_is_air = nx < 0 || ny < 0 || nz < 0 || 
                                         nx >= WORLD_SIZE as i32 || ny >= WORLD_SIZE as i32 || nz >= WORLD_SIZE as i32 ||
                                         chunk.data[get_idx(nx as usize, ny as usize, nz as usize)] == BlockTypes::AIR;

                    if neighbor_is_air {
                        let base_index = vertices.len() as u32;
                        for v in face.verts {
                            let packed: u32 = pack_information(face.normal, block);
                            vertices.push(Vertex {
                                position: [
                                    (x as i32 + v[0]),
                                    (y as i32 + v[1]),
                                    (z as i32 + v[2])
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