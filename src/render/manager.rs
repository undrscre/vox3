use std::collections::HashMap;

use cgmath::Vector3;
use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;
use crate::{engine::{data::Vertex, meshgen::Mesh}, game::chunk::BuildIdentityHasher, render::device::GPUDevice};

pub struct ResourceManager {
    pub meshes: FxHashMap<u64, GPUMesh>,
    pub debug_meshes: FxHashMap<u64, GPUMesh>,
    pub megabuffer: Megabuffer
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device) -> Self {
        Self { 
            meshes: FxHashMap::default(), 
            debug_meshes: FxHashMap::default(),
            megabuffer: Megabuffer::new(device)
        }
    }

    pub fn update_chunk_mesh(&mut self, queue: &wgpu::Queue, key: u64, mesh_data: &Mesh) {
        let gpu_mesh = self.megabuffer.allocate_and_write(queue, mesh_data);
        self.meshes.insert(key, gpu_mesh);
    }

    pub fn unload_chunk(&mut self, key: u64) {
        self.meshes.remove(&key);
    }
}

pub struct GPUMesh {
    pub base_vertex: i32,
    pub first_index: u32,
    pub index_count: u32,
    pub world_pos: cgmath::Vector3<f32>
}

// megabuffer for terrain drawcalls
pub struct Megabuffer {
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub next_vertex_offset: u64,
    pub next_index_offset: u64
}

impl Megabuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        const SIZE: u64 = 1024; // in megabytes
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

        Self { vertex_buf, index_buf, next_vertex_offset: 0, next_index_offset: 0 }
    }

    pub fn allocate_and_write(&mut self, queue: &wgpu::Queue, mesh: &Mesh) -> GPUMesh {
        let v_offset = self.next_vertex_offset;
        let i_offset = self.next_index_offset;

        queue.write_buffer(&self.vertex_buf, v_offset, bytemuck::cast_slice(&mesh.vertices));
        queue.write_buffer(&self.index_buf, i_offset, bytemuck::cast_slice(&mesh.indices));

        let gpu_mesh = GPUMesh {
            base_vertex: (v_offset / std::mem::size_of::<Vertex>() as u64) as i32,
            first_index: (i_offset / 4) as u32,
            index_count: mesh.indices.len() as u32,
            world_pos: mesh.world_pos.cast().unwrap(),
        };

        self.next_vertex_offset += (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as u64;
        self.next_index_offset += (mesh.indices.len() * 4) as u64;

        gpu_mesh
    }
}