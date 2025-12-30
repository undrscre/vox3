use wgpu::util::DeviceExt;

use crate::engine::data::Vertex;

// Yes It Is Evil How I Have 2 Mesh Structs
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

// hello triangle!
impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: vec![
                Vertex { position: [-1, -1, 0], packed: 0 },
                Vertex { position: [1 , -1, 0], packed: 0 },
                Vertex { position: [0 , 1 , 0], packed: 0 }
            ],
            indices: vec![0, 1, 2]
        }
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