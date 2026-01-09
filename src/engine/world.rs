use super::data::BlockTypes;
use rand::Rng;

// simple world implementation
// @TODO: make this moddable ? maybe
//        & move this to data.rs
pub const WORLD_SIZE: usize = 32;
const TOTAL_SIZE: usize = WORLD_SIZE * WORLD_SIZE * WORLD_SIZE;

pub struct World {
    pub data: Vec<BlockTypes>
}

impl World {
    // default flat world
    pub fn new() -> Self {
        let mut data = vec![BlockTypes::AIR; TOTAL_SIZE];
        
        for z in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let mut rng = rand::thread_rng();
                
                for y in 0..WORLD_SIZE {
                    let pos = get_idx(x, y, z);
                    if rng.gen_bool(0.3) { // 30% chance for a block
                        data[pos] = BlockTypes::STONE;
                    }
                }
            }
        }

        Self {
            data
        }
    }
}

#[inline]
pub fn get_idx(x: usize, y: usize, z: usize) -> usize {
    (x * WORLD_SIZE * WORLD_SIZE) + (y * WORLD_SIZE) + z
}