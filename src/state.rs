use std::sync::Arc;
use winit::window::Window;

pub struct State {
    window: Arc<Window>
    // renderer ..

}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        Self {
            window
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