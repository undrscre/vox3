use std::{f32, sync::Arc};

use cgmath::{Point3, num_traits::Pow};
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};

use crate::{engine::{data::{BlockId, CHUNK_SIZE, ChunkCoords}, mods::blocks::BlockRegistry}, game::{Chunk, world::{WorldConfig, biomes::*}}};

const STEP: usize = 8;
const GRID_SIZE: usize = (CHUNK_SIZE / STEP) + 1;

#[derive(Clone, Debug)]
pub enum GenerationType {
    Flat,
    Test
}

pub struct WorldGenerator {
    pub noise: FastNoiseLite,
    pub registry: BlockRegistry,
    pub biomes: Vec<Arc<dyn Biome>>,
}

impl WorldGenerator {
    pub fn new(config: &WorldConfig, block_registry: BlockRegistry) -> Self {
        let mut noise = FastNoiseLite::with_seed(config.seed);
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.set_fractal_type(Some(FractalType::FBm));
        noise.set_fractal_octaves(Some(5));
        noise.set_frequency(Some(0.005));

        Self { 
            noise,
            registry: block_registry,
            biomes: vec![Arc::new(Plains)]
        }
    }

    pub fn generate(&self, chunk_pos: Point3<ChunkCoords>) -> Chunk {
        let mut chunk = Chunk::new(None, chunk_pos);
        let biome = self.select_biome(chunk_pos.x as f32, chunk_pos.y as f32);
        let (base_height, intensity) = biome.get_params();

        // calculate grid size and lerp through them
        let mut samples = vec![0.0f32; GRID_SIZE * GRID_SIZE * GRID_SIZE];

        for si in 0..GRID_SIZE {
            for sj in 0..GRID_SIZE {
                for sk in 0..GRID_SIZE {
                    let wx = (chunk_pos.x * CHUNK_SIZE as i32) + (si * STEP) as i32;
                    let wy = (chunk_pos.y * CHUNK_SIZE as i32) + (sk * STEP) as i32;
                    let wz = (chunk_pos.z * CHUNK_SIZE as i32) + (sj * STEP) as i32;

                    let noise = self.noise.get_noise_3d(wx as f32, wy as f32, wz as f32);
                    let idx = (sk * GRID_SIZE * GRID_SIZE) + (sj * GRID_SIZE) + si;
                    samples[idx] = (base_height as i32 - wy) as f32 + (noise * intensity as f32);
                }
            }
        }

        for x in 0..CHUNK_SIZE {
            let si = x / STEP;
            let tx = (x % STEP) as f32 / STEP as f32;
            for z in 0..CHUNK_SIZE {
                let sj = z / STEP;
                let tz = (z % STEP) as f32 / STEP as f32;

                for y in 0..CHUNK_SIZE {
                    let sk = y / STEP;
                    let ty = (y % STEP) as f32 / STEP as f32;

                    let c000 = samples[(sk * GRID_SIZE * GRID_SIZE) + (sj * GRID_SIZE) + si];
                    let c100 = samples[(sk * GRID_SIZE * GRID_SIZE) + (sj * GRID_SIZE) + (si + 1)];
                    let c001 = samples[(sk * GRID_SIZE * GRID_SIZE) + ((sj + 1) * GRID_SIZE) + si];
                    let c101 = samples[(sk * GRID_SIZE * GRID_SIZE) + ((sj + 1) * GRID_SIZE) + (si + 1)];
                    
                    let c010 = samples[((sk + 1) * GRID_SIZE * GRID_SIZE) + (sj * GRID_SIZE) + si];
                    let c110 = samples[((sk + 1) * GRID_SIZE * GRID_SIZE) + (sj * GRID_SIZE) + (si + 1)];
                    let c011 = samples[((sk + 1) * GRID_SIZE * GRID_SIZE) + ((sj + 1) * GRID_SIZE) + si];
                    let c111 = samples[((sk + 1) * GRID_SIZE * GRID_SIZE) + ((sj + 1) * GRID_SIZE) + (si + 1)];

                    let density = lerp_3d(c000, c100, c001, c101, c010, c110, c011, c111, tx, ty, tz);

                    if density > 0.0 {
                        let dx = (c100 - c000);
                        let dz = (c001 - c000);
                        let slope = (dx * dx + dz * dz).sqrt();
                        
                        let depth = 0;
                        chunk.set(x, y, z, biome.get_block(&self.registry, depth, slope));
                    }
                }
            }
        }

        chunk
    }

    fn select_biome(&self, x: f32, y: f32) -> &Arc<dyn Biome> {
        let biome_noise = self.noise.get_noise_2d(x * 0.1, y * 0.1);
        let idx = ((biome_noise + 1.) * 0.5 * (self.biomes.len() as f32 - 0.001)) as usize;
        &self.biomes[idx]
    }
}

// Fucking Evil Functions
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn get_slope(samples: Vec<f32>, x: usize, y: usize, z: usize) -> f32 {
    let slice = STEP * STEP;
    
    let current_idx = (y * slice) + (z * STEP) + x;
    let next_x_idx = (y * slice) + (z * STEP) + (x + 1);
    let next_z_idx = (y * slice) + ((z + 1) * STEP) + x;

    let dx = samples[next_x_idx] - samples[current_idx];
    let dz = samples[next_z_idx] - samples[current_idx];

    (dx * dx + dz * dz).sqrt()
}

// i dont know where i got this from
fn hash_fast(x: u32, y: u32, z: u32) -> f32 {
    let mut state = x.wrapping_mul(747796405).wrapping_add(2891336453);
    state = (state ^ (y.wrapping_mul(123456789))).rotate_left(13);
    state = (state ^ (z.wrapping_mul(987654321))).rotate_left(17);
    
    state = state.wrapping_mul(277803737).wrapping_add(2891336453);
    state = (state ^ (state >> 16)).wrapping_mul(277803737);
    
    (state as f32) / (u32::MAX as f32)
}

fn lerp_3d(
    c000: f32, c100: f32, c001: f32, c101: f32,
    c010: f32, c110: f32, c011: f32, c111: f32,
    tx: f32, ty: f32, tz: f32
) -> f32 {
    let x00 = lerp(c000, c100, tx);
    let x10 = lerp(c010, c110, tx);
    let x01 = lerp(c001, c101, tx);
    let x11 = lerp(c011, c111, tx);

    let z0 = lerp(x00, x01, tz);
    let z1 = lerp(x10, x11, tz);

    lerp(z0, z1, ty)
}