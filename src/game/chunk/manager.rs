use std::sync::Arc;

use cgmath::{Point3, Vector3};
use rustc_hash::FxHashSet;
use crate::{
    engine::{
        data::{ChunkCoords, pack_chunk_coords, unpack_chunk_coords, world_to_chunk}, 
        meshgen::{Mesh, mesh_chunk}, mods::blocks::BlockRegistry
    }, 
    game::{Chunk, world::{World, WorldGenerator}}, 
    render::{device::GPUDevice, manager::ResourceManager}
};

pub struct ChunkManager {
    pub gen_distance: i32,
    pub last_player_chunk: Point3<ChunkCoords>,

    chunk_tx: std::sync::mpsc::Sender<(Point3<ChunkCoords>, Chunk)>,
    chunk_rx: std::sync::mpsc::Receiver<(Point3<ChunkCoords>, Chunk)>,

    mesh_tx: std::sync::mpsc::Sender<(u64, Mesh)>,
    mesh_rx: std::sync::mpsc::Receiver<(u64, Mesh)>,

    pending_chunks: FxHashSet<u64>,
    dirty_chunks: FxHashSet<u64>,

    pub block_registry: BlockRegistry
}

pub struct ChunkNeighborhood {
    pub center: Arc<Chunk>,
    pub neighbors: [Option<Arc<Chunk>>; 6],
}

impl ChunkManager {
    pub fn new(gen_distance: i32, block_registry: BlockRegistry) -> Self {
        let (chunk_tx, chunk_rx) = std::sync::mpsc::channel();
        let (mesh_tx, mesh_rx) = std::sync::mpsc::channel();

        Self {
            gen_distance,
            last_player_chunk: Point3 {x: 0, y: 0, z: 0},
            chunk_tx,
            chunk_rx,
            mesh_tx,
            mesh_rx,
            pending_chunks: FxHashSet::default(),
            dirty_chunks: FxHashSet::default(),
            block_registry
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
                    let _ = tx.send((cmd, new_chunk));
                });
            }
        }

        while let Ok((cmd, new_chunk)) = self.chunk_rx.try_recv() {
            let key = pack_chunk_coords(cmd.x, cmd.y, cmd.z);
            self.pending_chunks.remove(&key);
  
            world.chunks.insert(key, Arc::new(new_chunk));
            self.mark_area_dirty(cmd);
        }

        self.process_mesh_queue(world, gpu, resource_manager);
        self.unload_distant_chunks(world, resource_manager);
    }

    fn mark_area_dirty(&mut self, position: Point3<ChunkCoords>) {
        let center_key = pack_chunk_coords(position.x, position.y, position.z);
        self.dirty_chunks.insert(center_key);

        let offsets = [
            Vector3::new(0, 0, 1),  // 0: +z (front)
            Vector3::new(0, 0, -1), // 1: -z (back)
            Vector3::new(1, 0, 0),  // 2: +x (right)
            Vector3::new(-1, 0, 0), // 3: -x (left)
            Vector3::new(0, 1, 0),  // 4: +y (top)
            Vector3::new(0, -1, 0), // 5: -y (bottom)
        ];

        for offset in offsets {
            let neighbor_position = position + offset;
            let neighbor_key = pack_chunk_coords(
                neighbor_position.x, 
                neighbor_position.y, 
                neighbor_position.z
            );
            self.dirty_chunks.insert(neighbor_key);
        }
    }

    fn try_get_neighborhood(&self, world: &World, position: Point3<ChunkCoords>) -> Option<ChunkNeighborhood> {
        let center = world.chunks.get(&pack_chunk_coords(position.x, position.y, position.z))?.clone();
        let mut neighbors: [Option<Arc<Chunk>>; 6] = [None, None, None, None, None, None];
        let offsets = [
            Vector3::new(0, 0, 1),  // 0: +z (front)
            Vector3::new(0, 0, -1), // 1: -z (back)
            Vector3::new(1, 0, 0),  // 2: +x (right)
            Vector3::new(-1, 0, 0), // 3: -x (left)
            Vector3::new(0, 1, 0),  // 4: +y (top)
            Vector3::new(0, -1, 0), // 5: -y (bottom)
        ];

        for i in 0..6 {
            let neighbor_pos = position + offsets[i];
            let key = pack_chunk_coords(neighbor_pos.x, neighbor_pos.y, neighbor_pos.z);
            neighbors[i] = world.chunks.get(&key).cloned();
        }

        Some(ChunkNeighborhood { center, neighbors })
    }

    fn process_mesh_queue(&mut self, world: &mut World, gpu: &GPUDevice, resource_manager: &mut ResourceManager) {
        let to_mesh: Vec<u64> = self.dirty_chunks.iter().cloned().collect();

        for key in to_mesh {
            let position = unpack_chunk_coords(key);

            if let Some(neighborhood) = self.try_get_neighborhood(world, position) {
                let tx = self.mesh_tx.clone();
                let registry = self.block_registry.clone();
                if neighborhood.neighbors.iter().any(|n| n.is_none()) {
                    continue; 
                }

                rayon::spawn(move || {
                    let mesh_data = mesh_chunk(&neighborhood, &registry);
                    let _ = tx.send((key, mesh_data));
                });

                self.dirty_chunks.remove(&key);
            }
        }

        let start_time = std::time::Instant::now();
        while let Ok((key, mesh_data)) = self.mesh_rx.try_recv() {
            resource_manager.update_chunk_mesh(&gpu.queue, key, &mesh_data);
            if start_time.elapsed().as_millis() > 2 { break; }
        }
    }

    fn unload_distant_chunks(&mut self, world: &mut World, resource_manager: &mut ResourceManager) {
        let player_chunk = self.last_player_chunk;
        let limit = self.gen_distance + 4;

        let to_remove: Vec<u64> = world.chunks.keys().filter(|&&key| {
            let position = unpack_chunk_coords(key);
            let dist_x = (position.x - player_chunk.x).abs();
            let dist_y = (position.y - player_chunk.y).abs();
            let dist_z = (position.z - player_chunk.z).abs();

            dist_x > limit || dist_y > limit || dist_z > limit
        }).cloned().collect();

        for key in to_remove {
            self.dirty_chunks.remove(&key);
            world.chunks.remove(&key);
            self.pending_chunks.remove(&key);
            resource_manager.unload_chunk(key);
        }
    }
}