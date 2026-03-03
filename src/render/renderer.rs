use std::collections::HashMap;

use log::info;

use crate::{engine::meshgen::mesh_chunk, game::{Chunk, world::World}, render::{manager::{GPUMesh, ResourceManager}, pipeline::PipelineType}};

use super::{
    device::GPUDevice,
    pipeline::Pipeline
};

pub struct Renderer {
    pub pipelines: HashMap<PipelineType, Pipeline>,
    pub resource_manager: ResourceManager
}

impl Renderer {
    pub fn new(pipelines: HashMap<PipelineType, Pipeline>) -> Self {
        Self {pipelines, resource_manager: ResourceManager::new() }
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
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }), 
                timestamp_writes: None, 
                occlusion_query_set: None 
            });

            if let Some(pipeline) = self.pipelines.get(&PipelineType::Default) {
                render_pass.set_pipeline(&pipeline.render_pipeline);
                render_pass.set_bind_group(0, &pipeline.camera_bind_group, &[]);

                for mesh in self.resource_manager.meshes.values() {
                    if mesh.index_count == 0 { continue; }

                    render_pass.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
                    render_pass.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
                }
            } else {
                panic!("default pipeline isn't inserted into the renderer; panicking now")
            }

            if let Some(debug_pipeline) = self.pipelines.get(&PipelineType::DebugWireframe) {
                render_pass.set_pipeline(&debug_pipeline.render_pipeline);
                render_pass.set_bind_group(0, &debug_pipeline.camera_bind_group, &[]);
                
                for debug_mesh in self.resource_manager.debug_meshes.values() {
                    render_pass.set_vertex_buffer(0, debug_mesh.vertex_buf.slice(..));
                    render_pass.set_index_buffer(debug_mesh.index_buf.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..debug_mesh.index_count, 0, 0..1);
                }
            }
        }

        gpu.queue.submit(Some(encoder.finish()));
        output.present();
        
        Ok(())
    }
}