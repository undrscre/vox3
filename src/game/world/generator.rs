use cgmath::Point3;
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};

use crate::{engine::data::{BlockTypes, CHUNK_SIZE, ChunkCoords}, game::{Chunk, world::{WorldConfig, generator}}};

pub enum GenerationType {
    Flat,
    Test
}

pub struct WorldGenerator {
    pub noise: FastNoiseLite
}

impl WorldGenerator {
    pub fn new(config: &WorldConfig) -> Self {
        let mut noise = FastNoiseLite::with_seed(config.seed);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.set_fractal_type(Some(FractalType::FBm));
        noise.set_fractal_octaves(Some(5));
        noise.set_frequency(Some(0.002));
        
        Self { 
            noise
        }
    }

    pub fn generate(&self, config: &WorldConfig, chunk_position: Point3<ChunkCoords>) -> Chunk {
        let mut chunk = Chunk::new(None, chunk_position);

        // todo: refactor later lol
        match config.generator {
            GenerationType::Flat => {
                if chunk_position.y == 0 {
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let block = if y > 5 {
                                    BlockTypes::STONE
                                } else {
                                    BlockTypes::AIR
                                };
                                chunk.set(x, y, z, block);
                            }
                        }    
                    }
                }
            }

            GenerationType::Test => {
                let chunk_min_y = chunk_position.y * CHUNK_SIZE as i32;

                let mut air_count = 0;
                let mut stone_count = 0;

                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let world_x = (chunk_position.x * CHUNK_SIZE as i32) + x as i32;
                        let world_z = (chunk_position.z * CHUNK_SIZE as i32) + z as i32;

                        let n = self.noise.get_noise_2d(world_x as f32, world_z as f32);
                        let height = ((n + 1.0) * 256.0) as i32; 

                        for y in 0..CHUNK_SIZE {
                            let world_y = chunk_min_y + y as i32;

                            let block = if world_y < height {
                                stone_count += 1;
                                BlockTypes::STONE
                            } else {
                                air_count += 1;
                                BlockTypes::AIR
                            };
                            
                            chunk.set(x, y, z, block);
                        }
                    }    
                }

                chunk.is_empty = stone_count == 0;
                chunk.is_solid = air_count == 0;
            }
            // _ => { unimplemented!() }
        }

        chunk
    }
}