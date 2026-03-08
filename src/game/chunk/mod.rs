use std::hash::{BuildHasher, Hasher};
use cgmath::{Point3};
use crate::engine::{data::{BlockTypes, CHUNK_SIZE, ChunkCoords, Vertex}, meshgen::Mesh};

// pub mod loader;
pub mod manager;

pub use manager::ChunkManager;

// how the hell do i structure things ...
pub struct Chunk {
    pub position: Point3<ChunkCoords>,
    pub data: Vec<BlockTypes>,
    pub is_empty: bool,
    pub is_solid: bool,
}

impl Chunk {
    pub fn new(fill: Option<BlockTypes>, pos: Point3<ChunkCoords>) -> Self {
        let total_size = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
        let mut data = vec![BlockTypes::AIR; total_size];

        let mut is_solid = false;
        let mut is_empty = true;

        if let Some(block) = fill {
            if block != BlockTypes::AIR {
                data.fill(block);
                is_empty = false;
                is_solid = true;
            }
        }

        Self { 
            position: pos, 
            data,
            is_empty,
            is_solid,
        }
    }

    // pub fn create_debug_mesh(pos: Point3<i32>, is_empty: bool, is_solid: bool) -> Mesh {
    //     let s = CHUNK_SIZE as i32;
    //     let ox = pos.x * s;
    //     let oy = pos.y * s;
    //     let oz = pos.z * s;
    //     let packed = 0;

    //     let vertices = vec![
    //         Vertex { position: [ox,     oy,     oz    ], packed }, // 0
    //         Vertex { position: [ox + s, oy,     oz    ], packed }, // 1
    //         Vertex { position: [ox + s, oy + s, oz    ], packed }, // 2
    //         Vertex { position: [ox,     oy + s, oz    ], packed }, // 3
    //         Vertex { position: [ox,     oy,     oz + s], packed }, // 4
    //         Vertex { position: [ox + s, oy,     oz + s], packed }, // 5
    //         Vertex { position: [ox + s, oy + s, oz + s], packed }, // 6
    //         Vertex { position: [ox,     oy + s, oz + s], packed }, // 7
    //     ];

    //     let indices: Vec<u32> = vec![
    //         0, 1, 1, 2, 2, 3, 3, 0, // bottom
    //         4, 5, 5, 6, 6, 7, 7, 4, // top
    //         0, 4, 1, 5, 2, 6, 3, 7  // pillars
    //     ];

    //     Mesh { vertices, indices }
    // }

    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockTypes {
        self.data[Self::idx(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: BlockTypes) {
        self.data[Self::idx(x, y, z)] = block;
        self.is_empty = false;
    }

    #[inline]
    fn idx(x: usize, y: usize, z: usize) -> usize {
        (x * CHUNK_SIZE * CHUNK_SIZE) + (z * CHUNK_SIZE) + y
    }
}

// hash stuff ...!!
// todo maybe decouple this from chunk.rs
#[derive(Default)]
pub struct IdentityHasher {
    state: u64
}

impl Hasher for IdentityHasher {
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.state = i
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Hello   No");
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(Default)]
pub struct BuildIdentityHasher;

impl BuildHasher for BuildIdentityHasher {
    type Hasher = IdentityHasher;
    fn build_hasher(&self) -> Self::Hasher {
        IdentityHasher::default()
    }
}
