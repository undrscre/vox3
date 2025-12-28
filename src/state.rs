use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use crate::render::{device::GPUDevice, pipeline::Pipeline, renderer::Renderer};

pub struct State {
    pub window: Arc<Window>,
    device: GPUDevice,

    // renderer related
    renderer: Renderer,
    pipeline: Pipeline,
    pub size: PhysicalSize<u32>
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let device = GPUDevice::new(window.clone()).await;
        let renderer = Renderer::new(&device);
        let pipeline = Pipeline::new(&device);
        let size = window.inner_size();

        Self {
            window,
            device,
            renderer,
            pipeline,
            size
        }
    }

    // render out the game state
    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render_frame(&self.device)
    }

    // kjlsfdgkjlsfdakl
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.device.resize(new_size);
        }
    }

    // handle input
    pub fn input() {
        todo!()
    }

    // update gamestate
    pub fn update(&self) {
        self.window.request_redraw();
    }
}