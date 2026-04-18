use std::sync::Arc;

pub mod biomes;
pub mod generator;

use cgmath::Point3;
pub use generator::{GenerationType, WorldGenerator};
use rustc_hash::FxHashMap;

use crate::{engine::data::{BlockId, pack_chunk_coords}, game::chunk::Chunk};

// eventually switch out for a more moddable approach lol

type ChunkMap = FxHashMap<u64, Arc<Chunk>>;

pub struct World {
    pub metadata: WorldConfig,
    pub chunks: ChunkMap,
}

#[derive(Clone, Debug)]
pub struct WorldConfig {
    pub seed: i32,
    pub generator: GenerationType,
    pub version: u8 // usually zero as of writing lol
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self { seed: rand::random::<i32>(), generator: GenerationType::Test, version: 0 }
    }
}

impl World {
    pub fn new(config: WorldConfig) -> Self {
        Self {
            metadata: config,
            chunks: FxHashMap::default(),
        }
    }

    pub fn get_block_at(&self, pos: Point3<f32>) -> BlockId {
        let gx = pos.x.floor() as i32;
        let gy = pos.y.floor() as i32;
        let gz = pos.z.floor() as i32;

        let cx = gx.div_euclid(32);
        let cy = gy.div_euclid(32);
        let cz = gz.div_euclid(32);

        let lx = gx.rem_euclid(32) as usize;
        let ly = gy.rem_euclid(32) as usize;
        let lz = gz.rem_euclid(32) as usize;

        let key = pack_chunk_coords(cx, cy, cz);
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.get(lx, ly, lz)
        } else {
            BlockId::AIR
        }
    }
}