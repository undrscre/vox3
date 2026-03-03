use std::collections::HashSet;

pub mod generator;
use cgmath::Point3;
pub use generator::{GenerationType, WorldGenerator};
use rustc_hash::FxHashMap;

use crate::{engine::data::{ChunkCoords, pack_chunk_coords}, game::chunk::Chunk};

// eventually switch out for a more moddable approach lol

type ChunkMap = FxHashMap<u64, Chunk>;

pub struct World {
    pub metadata: WorldConfig,
    pub chunks: ChunkMap,
    pub dirty_chunks: HashSet<u64> // positions of chunks to be meshed
}

#[derive(Clone)]
pub struct WorldConfig {
    pub seed: i32,
    pub generator: GenerationType,
    pub version: u8 // usually zero as of writing lol
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self { seed: 0, generator: GenerationType::Test, version: 0 }
    }
}

impl World {
    pub fn new(config: WorldConfig) -> Self {
        Self {
            metadata: config,
            chunks: FxHashMap::default(),
            dirty_chunks: HashSet::new()
        }
    }

    pub fn insert(&mut self, pos: Point3<ChunkCoords>, chunk: Chunk) {
        let key = pack_chunk_coords(pos.x, pos.y, pos.z);

        self.chunks.insert(key, chunk);
        self.dirty_chunks.insert(key);
        
        // todo: mark neighbors as dirty
    }
}