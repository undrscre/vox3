use fastnoise_lite::FastNoiseLite;

use crate::engine::{data::BlockId, mods::blocks::BlockRegistry};

pub trait Biome: Send + Sync {
    fn name(&self) -> &str;
    fn get_block(&self, registry: &BlockRegistry, depth: i32, slope: f32) -> BlockId;
    fn get_params(&self) -> (u32, u32); // noise params
}

pub struct Plains;
impl Biome for Plains {
    fn get_block(&self, registry: &BlockRegistry, depth: i32, slope: f32) -> BlockId {
        let dirt = registry.name_to_id["dirt"];
        BlockId(dirt)
    }

    fn name(&self) -> &str {
        "Plains"
    }

    fn get_params(&self) -> (u32, u32) {
        (256, 520)
    }
}