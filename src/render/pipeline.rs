use wgpu::{BindGroup, Buffer, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline, util::DeviceExt};
use crate::engine::{camera::CameraUniform, data::Vertex};

use super::device::GPUDevice;

pub struct Pipeline {
    pub render_pipeline: RenderPipeline,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer,
}

#[derive(Eq, Hash, PartialEq)]

pub enum PipelineType {
    Default,
    DebugWireframe,
    Sky,
}

impl Pipeline {
    fn camera_layout(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> (PipelineLayout, BindGroup, Buffer) {
        // camera bind group
        let camera_bind_group_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        (
            gpu
                .device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                }),
            camera_bind_group,
            camera_buffer
        )
    }

    pub fn default_pipeline(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> Self {
        // should be self explanatory yeah ? Good luck
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl"));
        
        let (layout, camera_bind_group, camera_buffer) = Self::camera_layout(gpu, camera_uniform);

        let render_pipeline = gpu.device
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

        Self { render_pipeline, camera_bind_group, camera_buffer }
    }

    pub fn debug_pipeline(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/debug.wgsl"));
        
        let (layout, camera_bind_group, camera_buffer) = Self::camera_layout(gpu, camera_uniform);

        let render_pipeline = gpu.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
                label: Some("debug pipeline"), 
                layout: Some(&layout), 
                vertex: wgpu::VertexState { 
                    module: &shader, 
                    entry_point: Some("dvs_main"), 
                    compilation_options: Default::default(), 
                    buffers: &[Vertex::layout()] 
                }, 
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("dfs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gpu.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }), 
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    polygon_mode: wgpu::PolygonMode::Line,
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState { 
                    format: crate::render::device::DEPTH_FORMAT, 
                    depth_write_enabled: true, 
                    depth_compare: wgpu::CompareFunction::GreaterEqual, 
                    stencil: wgpu::StencilState::default(), 
                    bias: wgpu::DepthBiasState {
                        constant: 2, 
                        slope_scale: 1.0,
                        clamp: 0.0,
                    },
                }), 
                multisample: wgpu::MultisampleState::default(), 
                multiview: None, 
                cache: Default::default() 
            });

        Self { render_pipeline, camera_bind_group, camera_buffer }
    }

    pub fn sky_pipeline(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> Self {
        let shader = gpu 
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/sky.wgsl"));

        let (layout, camera_bind_group, camera_buffer) = Self::camera_layout(gpu, camera_uniform);

        let render_pipeline = gpu.device
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

        Self { render_pipeline, camera_bind_group, camera_buffer }
    }
}