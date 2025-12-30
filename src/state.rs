use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use crate::render::{device::GPUDevice, meshman::{GPUMesh, Mesh}, pipeline::Pipeline, renderer::Renderer};

pub struct State {
    pub window: Arc<Window>,
    gpu: GPUDevice,

    // renderer related
    renderer: Renderer,
    pipeline: Pipeline,

    // yolo
    test_mesh: GPUMesh,
    pub size: PhysicalSize<u32>
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let gpu = GPUDevice::new(window.clone()).await;
        let renderer = Renderer::new(&gpu);
        let pipeline = Pipeline::new(&gpu);
        let size = window.inner_size();

        // test mesh
        let mesh = Mesh::default();
        let test_mesh = GPUMesh::from_mesh(&gpu.device, &mesh);

        Self {
            window,
            gpu,
            renderer,
            pipeline,
            size,
            test_mesh
        }
    }

    // render out the game state
    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render_frame(&self.gpu, &self.test_mesh)
    }

    // kjlsfdgkjlsfdakl
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.gpu.resize(new_size);
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