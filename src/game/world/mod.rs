use std::{collections::HashSet, sync::Arc};

pub mod generator;
use cgmath::Point3;
pub use generator::{GenerationType, WorldGenerator};
use rustc_hash::FxHashMap;

use crate::{engine::data::{ChunkCoords, pack_chunk_coords}, game::chunk::Chunk};

// eventually switch out for a more moddable approach lol

type ChunkMap = FxHashMap<u64, Arc<Chunk>>;

pub struct World {
    pub metadata: WorldConfig,
    pub chunks: ChunkMap,
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
        }
    }
}