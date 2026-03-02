use std::collections::HashSet;

use cgmath::{Point3, Vector3};

use crate::{engine::data::{ChunkCoords, world_to_chunk}, game::chunk::BuildIdentityHasher};

pub struct ChunkManager {
    pub gen_distance: i32,
    pub last_player_chunk: Point3<ChunkCoords>,
}

impl ChunkManager {
    pub fn new(gen_distance: i32) -> Self {
        Self {
            gen_distance,
            last_player_chunk: Point3 {x: 0, y: 0, z: 0},
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

        commands
    }
}