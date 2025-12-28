use super::{
    device::GPUDevice,
    pipeline::Pipeline
};

pub struct Renderer {
    pub pipeline: Pipeline,
}

impl Renderer {
    pub fn new(gpu: &GPUDevice) -> Self {
        let pipeline = Pipeline::new(gpu);
        Self {pipeline}
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store
                    },
                    depth_slice: None,
                })], 
                depth_stencil_attachment: None, 
                timestamp_writes: None, 
                occlusion_query_set: None 
            });

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        gpu.queue.submit(Some(encoder.finish()));
        output.present();
        
        Ok(())
    }
}