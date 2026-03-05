use std::f32;

use cgmath::{Matrix, Matrix2, Point3, num_traits::Pow};
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};

use crate::{engine::data::{BlockTypes, CHUNK_SIZE, ChunkCoords}, game::{Chunk, world::{WorldConfig, generator}}};

#[derive(Clone, Debug)]
pub enum GenerationType {
    Flat,
    Test
}

pub struct WorldGenerator {
    pub noise: FastNoiseLite,
    pub config: WorldConfig
}

impl WorldGenerator {
    pub fn new(config: &WorldConfig) -> Self {
        let mut noise = FastNoiseLite::with_seed(config.seed);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.set_fractal_type(Some(FractalType::FBm));
        noise.set_fractal_octaves(Some(5));
        noise.set_frequency(Some(0.002));
        
        Self { 
            noise,
            config: config.clone()
        }
    }

    pub fn generate(&self, chunk_position: Point3<ChunkCoords>) -> Chunk {
        let mut chunk = Chunk::new(None, chunk_position);

        // todo: refactor later lol
        match self.config.generator {
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
                let step = 4;
                let grid_size = (CHUNK_SIZE / step) + 1;

                let mut stone_count = 0;
                let mut air_count = 0;

                // presample with a 2d grid
                let mut samples = vec![vec![0.0f32; grid_size]; grid_size];
                for i in 0..grid_size {
                    for j in 0..grid_size {
                        let wx = (chunk_position.x * CHUNK_SIZE as i32) + (i * step) as i32;
                        let wz = (chunk_position.z * CHUNK_SIZE as i32) + (j * step) as i32;
                        samples[i][j] = (self.noise.get_noise_2d(wx as f32, wz as f32) + 1.0).pow(2.) * 100.0;
                    }
                }

                // lerp through the grid
                for x in 0..CHUNK_SIZE {
                    let i = x / step;
                    let tx = (x % step) as f32 / step as f32;
                    for z in 0..CHUNK_SIZE {
                        let j = z / step;
                        let tz = (z % step) as f32 / step as f32;

                        let height = lerp_2d(
                            samples[i][j], samples[i][j+1],
                            samples[i+1][j], samples[i+1][j+1],
                            tx, tz
                        ) as i32;

                        for y in 0..CHUNK_SIZE {
                            let world_y = (chunk_position.y * CHUNK_SIZE as i32) + y as i32;
                            let block = if world_y < height { stone_count += 1; BlockTypes::STONE } 
                                        else { air_count += 1; BlockTypes::AIR };
                        
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

// lerp func

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn lerp_2d(h00: f32, h01: f32, h10: f32, h11: f32, tx: f32, tz: f32) -> f32 {
    // get the bottom and top edge
    let bottom = lerp(h00, h10, tx);
    let top = lerp(h01, h11, tx);

    // lerp between those two
    lerp(bottom, top, tz)
}