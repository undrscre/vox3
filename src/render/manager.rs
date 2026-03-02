use std::collections::HashMap;

use wgpu::util::DeviceExt;
use crate::{engine::meshgen::Mesh, game::chunk::BuildIdentityHasher};

pub struct ResourceManager {
    pub meshes: HashMap<u64, GPUMesh, BuildIdentityHasher>,
    pub debug_meshes: HashMap<u64, GPUMesh, BuildIdentityHasher>
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { 
            meshes: HashMap::with_hasher(BuildIdentityHasher::default()), 
            debug_meshes: HashMap::with_hasher(BuildIdentityHasher::default()) 
        }
    }

    pub fn update_chunk_mesh(&mut self, device: &wgpu::Device, key: u64, mesh_data: &Mesh) {
        let gpu_mesh = GPUMesh::from_mesh(&device, mesh_data);
        self.meshes.insert(key, gpu_mesh);
    }

    pub fn update_debug_mesh(&mut self, device: &wgpu::Device, key: u64, mesh_data: &Mesh) {
        let gpu_mesh = GPUMesh::from_mesh(&device, mesh_data);
        self.debug_meshes.insert(key, gpu_mesh);
    }

    pub fn unload_chunk(&mut self, key: u64) {
        self.meshes.remove(&key);
    }
}

// handle vertex and index buffers
pub struct GPUMesh {
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub index_count: u32,
}

impl GPUMesh {
    pub fn from_mesh(device: &wgpu::Device, mesh: &Mesh) -> Self {
        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("idx buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buf,
            index_buf,
            index_count: mesh.indices.len() as u32,
        }
    }
}