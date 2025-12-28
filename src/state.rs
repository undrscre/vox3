use std::sync::Arc;
use winit::window::Window;

use crate::render::device::GPUDevice;

pub struct State {
    window: Arc<Window>,
    device: GPUDevice,
    // renderer ..

}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let device = GPUDevice::new(window.clone()).await;
        Self {
            window,
            device
        }
    }

    // render out the game state
    pub fn render(&self) {
        // wawa
    }

    // kjlsfdgkjlsfdakl
    pub fn resize() {
        todo!()
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