use log::info;

use crate::{engine::meshgen::mesh_chunk, game::world::World, render::manager::{GPUMesh, ResourceManager}};

use super::{
    device::GPUDevice,
    pipeline::Pipeline
};

pub struct Renderer {
    pub pipeline: Pipeline,
    pub resource_manager: ResourceManager
}

impl Renderer {
    pub fn new(pipeline: Pipeline) -> Self {
        Self {pipeline, resource_manager: ResourceManager::new() }
    }

    pub fn render_frame(&self, gpu: &GPUDevice) -> Result<(), wgpu::SurfaceError> {
        let output = gpu.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("render_pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store
                    },
                    depth_slice: None,
                })], 
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &gpu.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }), 
                timestamp_writes: None, 
                occlusion_query_set: None 
            });

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.pipeline.camera_bind_group, &[]);

            for mesh in self.resource_manager.meshes.values() {
                if mesh.index_count == 0 { continue; }

                render_pass.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
                render_pass.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }

        gpu.queue.submit(Some(encoder.finish()));
        output.present();
        
        Ok(())
    }

    pub fn sync_world(&mut self, gpu: &GPUDevice, world: &mut World) {
        let dirty_chunks: Vec<u64> = world.dirty_chunks.drain().collect();
        if !dirty_chunks.is_empty() {
            log::info!("meshing {} dirty chunks", dirty_chunks.len());
        }

        for key in dirty_chunks {
            if let Some(chunk) = world.chunks.get(&key) {
                if chunk.data.is_empty() {
                    continue;
                }
                let mesh_data = mesh_chunk(&chunk);
                self.resource_manager.update_chunk_mesh(&gpu.device, key, &mesh_data);
            }
        }
    }

}