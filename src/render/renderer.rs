use std::collections::HashMap;

use log::info;

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
        let mut stages: Vec<Box<dyn RenderPipelineTrait>> = Vec::new();

        stages.push(Box::new(DefaultPipeline::new(gpu)));

        Self {stages, resource_manager: ResourceManager::new() }
    }

    pub fn render_frame(&self, gpu: &GPUDevice) -> Result<(), wgpu::SurfaceError> {
        let output = gpu.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder")
        });

        for stage in &self.stages {
            stage.record(&mut encoder, &view, gpu, &self.resource_manager);
        }

        gpu.queue.submit(Some(encoder.finish()));
        output.present();
        
        Ok(())
    }
}