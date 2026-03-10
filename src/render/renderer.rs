use crate::{engine::frustum::Frustum, render::pipelines::SkyPipeline};

use super::{
    device::GPUDevice,
    pipelines::{RenderPipelineTrait, DefaultPipeline},
    manager::ResourceManager
};

pub struct Renderer {
    pub stages: Vec<Box<dyn RenderPipelineTrait>>,
    pub resource_manager: ResourceManager
}

impl Renderer {
    pub fn new(gpu: &GPUDevice) -> Self {
        let mut stages: Vec<Box<dyn RenderPipelineTrait>> = vec![
            Box::new(DefaultPipeline::new(gpu)),
            Box::new(SkyPipeline::new(gpu))
        ];

        stages.sort_by_key(|s| s.priority());

        Self {stages, resource_manager: ResourceManager::new() }
    }

    pub fn render_frame(&self, gpu: &GPUDevice, frustum: &Frustum) -> Result<(), wgpu::SurfaceError> {
        let output = gpu.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder")
        });

        for stage in &self.stages {
            stage.record(&mut encoder, &view, gpu, &self.resource_manager, frustum);
        }

        gpu.queue.submit(Some(encoder.finish()));
        output.present();
        
        Ok(())
    }
}