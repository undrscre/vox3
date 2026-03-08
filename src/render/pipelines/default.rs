use wgpu::{BindGroup, Buffer, RenderPipeline};

use crate::engine::camera::CameraUniform;
use crate::engine::data::{CHUNK_SIZE, Vertex};
use crate::engine::frustum::Frustum;
use crate::render::pipelines::RenderPipelineTrait;
use crate::render::{
    device::GPUDevice,
    pipelines::create_camera_layout
};

pub struct DefaultPipeline {
    pub pipeline: RenderPipeline,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer,
}

impl DefaultPipeline {
    pub fn new(gpu: &GPUDevice) -> Self {
        let (layout, camera_bind_group, camera_buffer) = create_camera_layout(gpu, &CameraUniform::new());

        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/main.wgsl"));

        let pipeline = gpu.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
                label: Some("default pipeline"), 
                layout: Some(&layout), 
                vertex: wgpu::VertexState { 
                    module: &shader, 
                    entry_point: Some("vs_main"), 
                    compilation_options: Default::default(), 
                    buffers: &[Vertex::layout()] 
                }, 
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gpu.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }), 
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                }, 
                depth_stencil: Some(wgpu::DepthStencilState { 
                    format: crate::render::device::DEPTH_FORMAT, 
                    depth_write_enabled: true, 
                    depth_compare: wgpu::CompareFunction::Greater, 
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                }), 
                multisample: wgpu::MultisampleState::default(), 
                multiview: None, 
                cache: Default::default() 
            });

        Self { pipeline, camera_bind_group, camera_buffer }
    }
}

impl RenderPipelineTrait for DefaultPipeline {
    fn update(&self, queue: &wgpu::Queue, camera: CameraUniform) {
        let binding = [camera];
        let data = bytemuck::cast_slice(&binding);

        queue.write_buffer(&self.camera_buffer, 0, data);
    }

    fn record<'a>(
            &'a self,
            encoder: &mut wgpu::CommandEncoder,
            view: &wgpu::TextureView,
            gpu: &'a GPUDevice,
            resources: &'a crate::render::manager::ResourceManager,
            frustum: &Frustum
        ) 
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("voxel render pass"), 
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        for mesh in resources.meshes.values() {
            if mesh.index_count == 0 { continue; }

            if !frustum.contains_chunk(mesh.world_pos, CHUNK_SIZE as f32) {
                continue;
            }

            render_pass.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
            render_pass.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
        }
    }

    fn reload_shader(&mut self, gpu: &GPUDevice) {
        todo!()
    }
}
