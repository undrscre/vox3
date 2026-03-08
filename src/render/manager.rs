use std::collections::HashMap;

use cgmath::Vector3;
use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;
use crate::{engine::meshgen::Mesh, game::chunk::BuildIdentityHasher};

pub struct ResourceManager {
    pub meshes: FxHashMap<u64, GPUMesh>,
    pub debug_meshes: FxHashMap<u64, GPUMesh>
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { 
            meshes: FxHashMap::default(), 
            debug_meshes: FxHashMap::default() 
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
        if let Some(mesh_entry) = self.meshes.remove(&key) {
            mesh_entry.vertex_buf.destroy(); 
            mesh_entry.index_buf.destroy();
        }
    }
}

// handle vertex and index buffers
pub struct GPUMesh {
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub index_count: u32,
    pub world_pos: cgmath::Vector3<f32>
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
            // fuck
            world_pos: cgmath::Vector3::new(
                mesh.world_pos.x as f32,
                mesh.world_pos.y as f32,
                mesh.world_pos.z as f32,
            ),
        }
    }
}