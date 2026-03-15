use std::ops::Range;

use rustc_hash::FxHashMap;
use crate::{engine::{data::Vertex, meshgen::Mesh}};

pub struct ResourceManager {
    pub meshes: FxHashMap<u64, GPUMesh>,
    pub megabuffer: Megabuffer
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device) -> Self {
        Self { 
            meshes: FxHashMap::default(), 
            megabuffer: Megabuffer::new(device)
        }
    }

    pub fn update_chunk_mesh(&mut self, queue: &wgpu::Queue, key: u64, mesh_data: &Mesh) {
        let gpu_mesh = self.megabuffer.allocate_and_write(queue, mesh_data);
        self.meshes.insert(key, gpu_mesh);
    }

    pub fn unload_chunk(&mut self, key: u64) {
        let mesh = self.meshes.remove(&key);
        if let Some(mesh) = mesh {
            self.megabuffer.free(mesh.allocation);
        }
    }
}

pub struct Allocation {
    pub v_range: Range<u64>,
    pub i_range: Range<u64>
}

pub struct GPUMesh {
    pub base_vertex: i32,
    pub first_index: u32,
    pub index_count: u32,
    pub world_pos: cgmath::Vector3<f32>,
    pub allocation: Allocation,
}

// megabuffer for terrain drawcalls
pub struct Megabuffer {
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,

    pub free_vertex_region: Vec<Range<u64>>,
    pub free_index_region: Vec<Range<u64>>,
    pub v_cursor: u64,
    pub i_cursor: u64,
}

impl Megabuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        const SIZE: u64 = 256; // in megabytes
        let vertex_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("terrain vertex megabuffer"),
            size: SIZE * 1024 * 1024,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let index_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("terrain index megabuffer"),
            size: SIZE * 1024 * 1024,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        Self { vertex_buf, index_buf, i_cursor: 0, v_cursor: 0, free_index_region: vec![], free_vertex_region: vec![]}
    }

    pub fn allocate_and_write(&mut self, queue: &wgpu::Queue, mesh: &Mesh) -> GPUMesh {
        let v_size = (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as u64;
        let i_size = (mesh.indices.len() * 4) as u64;

        let v_offset = Self::find_space(&mut self.free_vertex_region, v_size, &mut self.v_cursor);
        let i_offset = Self::find_space(&mut self.free_index_region, i_size, &mut self.i_cursor);

        queue.write_buffer(&self.vertex_buf, v_offset, bytemuck::cast_slice(&mesh.vertices));
        queue.write_buffer(&self.index_buf, i_offset, bytemuck::cast_slice(&mesh.indices));

        let gpu_mesh = GPUMesh {
            base_vertex: (v_offset / std::mem::size_of::<Vertex>() as u64) as i32,
            first_index: (i_offset / 4) as u32,
            index_count: mesh.indices.len() as u32,
            world_pos: mesh.world_pos.cast().unwrap(),
            allocation: Allocation { 
                v_range: v_offset..v_offset + v_size, 
                i_range: i_offset..i_offset + i_size
            }
        };

        gpu_mesh
    }

    fn find_space(regions: &mut Vec<Range<u64>>, size: u64, cursor: &mut u64) -> u64 {
        if let Some(pos) = regions.iter().position(|r| (r.end - r.start) >= size) {
            let hole = regions.remove(pos);
            let start = hole.start;
            if hole.end - hole.start > size {
                regions.push(hole.start + size..hole.end);
            }
            return start;
        }

        let start = *cursor;
        *cursor += size;
        start
    }

    pub fn free(&mut self, alloc: Allocation) {
        self.free_index_region.push(alloc.i_range);
        self.free_vertex_region.push(alloc.v_range);
    }
}