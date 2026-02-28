use std::collections::HashMap;
use cgmath::{Vector3, Zero};

use crate::game::chunk::{BuildIdentityHasher, Chunk};

type ChunkMap = HashMap<u64, Chunk, BuildIdentityHasher>;
pub struct World {
    pub chunks: ChunkMap
}

impl World {
    // stupid test impl
    pub fn new() -> Self {
        let mut chunks = ChunkMap::default();
        let chunk = Chunk::new(Some(super::data::BlockTypes::STONE), Vector3::zero());
        let key = Self::pack_coords(0, 0, 0);
        chunks.insert(key, chunk);
        World { chunks }
    }

    pub fn pack_coords(x: u16, y: u16, z: u16) -> u64 {
        ((z as u64) << 32) | ((y as u64) << 16) | (x as u64)
    }

    pub fn get_chunk(&self, x: u16, y: u16, z: u16) -> Option<&Chunk> {
        let key = Self::pack_coords(x, y, z);
        self.chunks.get(&key)
    }

    
}