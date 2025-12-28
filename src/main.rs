mod state;
mod render;

use std::sync::Arc;
use futures::executor::block_on;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

use crate::state::State;

fn main() {
    env_logger::init();
    log::info!("mrrpp");

    // initialize the window ..!! hopefully .. we pray
    let event_loop = EventLoop::new().expect("can't create event loop");
    let window = WindowBuilder::new()
        .with_title("Hello meong! .. 2!")
        .build(&event_loop)
        .expect("can't create window");

    // initialize gamestate
    event_loop.set_control_flow(ControlFlow::Poll);
    let state = block_on(State::new(Arc::new(window)));

    // execute through the eventloop
    let _ = event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                log::info!("Ok! Bye bye :)");
                target.exit();
            },
            Event::AboutToWait => {
                state.update();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                state.render();
            }
            _ => {},
        }
    });
}