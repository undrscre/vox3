use wgpu::{BindGroup, Buffer, RenderPipeline};

use crate::{engine::camera::CameraUniform, render::{device::GPUDevice, pipelines::{RenderPipelineTrait, create_camera_layout}}};

pub struct SkyPipeline {
    pub pipeline: RenderPipeline,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer,
}

impl SkyPipeline {
    pub fn new(gpu: &GPUDevice) -> Self {
        let shader = gpu 
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/sky.wgsl"));

        let (layout, camera_bind_group, camera_buffer) = create_camera_layout(gpu, &CameraUniform::new());

        let pipeline = gpu.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
                label: Some("sky pipeline"), 
                layout: Some(&layout), 
                vertex: wgpu::VertexState { 
                    module: &shader, 
                    entry_point: Some("svs_main"), 
                    compilation_options: Default::default(), 
                    buffers: &[], 
                }, 
                primitive: wgpu::PrimitiveState { 
                    topology: wgpu::PrimitiveTopology::TriangleStrip, 
                    cull_mode: None,
                    ..Default::default()
                }, 
                depth_stencil: Some(wgpu::DepthStencilState { 
                    format: crate::render::device::DEPTH_FORMAT, 
                    depth_write_enabled: false, 
                    depth_compare: wgpu::CompareFunction::Equal, 
                    stencil: wgpu::StencilState::default(), 
                    bias: wgpu::DepthBiasState::default()
                }), 
                multisample: wgpu::MultisampleState::default(), 
                fragment: Some(wgpu::FragmentState { 
                    module: &shader, 
                    entry_point: Some("sfs_main"), 
                    compilation_options: Default::default(), 
                    targets: &[Some(wgpu::ColorTargetState { 
                        format: gpu.config.format, 
                        blend: Some(wgpu::BlendState::REPLACE), 
                        write_mask: wgpu::ColorWrites::all() 
                    })]
                }), 
                multiview: None, 
                cache: None 
            });

        Self { pipeline, camera_bind_group, camera_buffer }
    }
}

impl RenderPipelineTrait for SkyPipeline {
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
            _resources: &'a crate::render::manager::ResourceManager,
            _frustum: &crate::engine::frustum::Frustum
        ) {
            let mut sky_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("sky render pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                    depth_slice: None
                })], 
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: &gpu.depth_texture.view, 
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Discard }), 
                    stencil_ops: None,
                }), 
                ..Default::default()
            });

            sky_pass.set_pipeline(&self.pipeline);
            sky_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // DRAW 4 VERTICES OUT OF THIN AIR. :yum:
            sky_pass.draw(0..4, 0..1);
        }
}