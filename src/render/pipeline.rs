use wgpu::{PipelineLayoutDescriptor, RenderPipeline, util::DeviceExt, BindGroup, Buffer};
use crate::engine::{camera::CameraUniform, data::Vertex};

use super::device::GPUDevice;

pub struct Pipeline {
    pub render_pipeline: RenderPipeline,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer
}

impl Pipeline {
    pub fn new(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> Self {
        // should be self explanatory yeah ? Good luck
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        // camera bind group
        let camera_bind_group_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let camera_buffer = gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cam_buffer"),
            contents: bytemuck::cast_slice(&[*camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        
        let render_pipeline = gpu.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
                label: Some("pipeline"), 
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
                primitive: wgpu::PrimitiveState::default(), 
                depth_stencil: None, 
                multisample: wgpu::MultisampleState::default(), 
                multiview: None, 
                cache: Default::default() 
            });

        Self { render_pipeline, camera_bind_group, camera_buffer }
    }
}