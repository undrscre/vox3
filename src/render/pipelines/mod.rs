use wgpu::{BindGroup, Buffer, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline, util::DeviceExt};
use crate::{engine::{camera::CameraUniform, data::Vertex, frustum::Frustum}, render::manager::ResourceManager};

use super::device::GPUDevice;

pub mod default;
pub use default::DefaultPipeline;
pub mod sky;
pub use sky::SkyPipeline;

pub trait RenderPipelineTrait {
    fn update(&self, queue: &wgpu::Queue, camera: CameraUniform);

    fn record<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        gpu: &'a GPUDevice,
        resources: &'a ResourceManager,
        frustum: &Frustum
    );

    fn reload_shader(&mut self, gpu: &GPUDevice) {
        todo!()
    }

    fn priority(&self) -> i32 { 0 }
}

pub fn create_camera_layout(gpu: &GPUDevice, camera_uniform: &CameraUniform) -> (PipelineLayout, BindGroup, Buffer) {
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
                    label: Some("camera bind group layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                }),
            camera_bind_group,
            camera_buffer
        )
    }