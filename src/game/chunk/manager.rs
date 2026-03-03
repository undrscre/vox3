use std::sync::Arc;

use cgmath::{Point3, Vector3};
use rustc_hash::FxHashSet;
use crate::{engine::{data::{ChunkCoords, pack_chunk_coords, world_to_chunk}, meshgen::{Mesh, mesh_chunk}}, game::{Chunk, world::{World, WorldGenerator}}, render::{device::GPUDevice, manager::ResourceManager, renderer::Renderer}};

pub struct ChunkManager {
    pub gen_distance: i32,
    pub last_player_chunk: Point3<ChunkCoords>,

    chunk_tx: std::sync::mpsc::Sender<(Point3<ChunkCoords>, Chunk, Option<Mesh>)>,
    chunk_rx: std::sync::mpsc::Receiver<(Point3<ChunkCoords>, Chunk, Option<Mesh>)>,
    pending_chunks: FxHashSet<u64>
}

impl ChunkManager {
    pub fn new(gen_distance: i32) -> Self {
        let (chunk_tx, chunk_rx) = std::sync::mpsc::channel();
        Self {
            gen_distance,
            last_player_chunk: Point3 {x: 0, y: 0, z: 0},
            chunk_tx,
            chunk_rx,
            pending_chunks: FxHashSet::default()
        }
    }

    pub fn get_load_commands(&mut self, player_pos: Point3<f32>) -> Vec<Point3<ChunkCoords>> {
        let player_chunk = world_to_chunk(player_pos);
        if player_chunk == self.last_player_chunk {
            return Vec::new();
        }
        self.last_player_chunk = player_chunk;

        let mut commands = Vec::new();
        for x in -self.gen_distance..=self.gen_distance {
            for y in -self.gen_distance..=self.gen_distance {
                for z in -self.gen_distance..=self.gen_distance {
                    let target = player_chunk + Vector3 {x, y, z};
                    commands.push(target);
                }
            }
        }

        commands.sort_by(|a, b| {
            let a_dx = a.x as i32 - player_chunk.x as i32;
            let a_dy = a.y as i32 - player_chunk.y as i32;
            let a_dz = a.z as i32 - player_chunk.z as i32;
            let a_dist_sq = a_dx*a_dx + a_dy*a_dy + a_dz*a_dz;
            
            let b_dx = b.x as i32 - player_chunk.x as i32;
            let b_dy = b.y as i32 - player_chunk.y as i32;
            let b_dz = b.z as i32 - player_chunk.z as i32;
            let b_dist_sq = b_dx*b_dx + b_dy*b_dy + b_dz*b_dz;
            
            a_dist_sq.cmp(&b_dist_sq)
        });
        commands
    }

    pub fn update(
        &mut self, 
        world: &mut World, 
        generator: &Arc<WorldGenerator>, 
        gpu: &GPUDevice, 
        resource_manager: &mut ResourceManager, 
        player_pos: Point3<f32>
    ) {
        let commands = self.get_load_commands(player_pos);

        for cmd in commands {
            let key = pack_chunk_coords(cmd.x, cmd.y, cmd.z);

            if !world.chunks.contains_key(&key) && !self.pending_chunks.contains(&key) {
                self.pending_chunks.insert(key);

                let tx = self.chunk_tx.clone();
                let generator = generator.clone();
                rayon::spawn(move || {
                    let new_chunk = generator.generate(cmd);
                    let mut mesh_data = None;
                    if !new_chunk.is_empty && !new_chunk.is_solid {
                        mesh_data = Some(mesh_chunk(&new_chunk));
                    }

                    let _ = tx.send((cmd, new_chunk, mesh_data));
                });
            }
        }

        let mut limit = 0;
        while let Ok((cmd, new_chunk, mesh_data)) = self.chunk_rx.try_recv() {
            let key = pack_chunk_coords(cmd.x, cmd.y, cmd.z);
            self.pending_chunks.remove(&key);

            world.chunks.insert(key, new_chunk);
            if let Some(mesh) = mesh_data {
                resource_manager.update_chunk_mesh(&gpu.device, key, &mesh);
            }

            limit += 1;
            if limit > 255 { println!("limit reached"); break; }
        }
    }
}