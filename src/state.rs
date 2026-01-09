use std::{collections::HashSet, sync::Arc, time::Instant};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{
    engine::{mesh::{Mesh, mesh_chunk}, player::Player, world::World}, 
    render::{device::GPUDevice, meshman::GPUMesh, pipeline::Pipeline, renderer::Renderer}};

pub struct State {
    pub window: Arc<Window>,
    gpu: GPUDevice,

    // renderer related
    renderer: Renderer,
    player: Player,

    // yolo
    test_mesh: GPUMesh,
    pub size: PhysicalSize<u32>,

    // todo AHHHHHHH decouple player event handling
    pressed_keys: HashSet<KeyCode>,
    last_update: Instant
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        // actual game shish
        let player = Player::new();

        let gpu = GPUDevice::new(window.clone()).await;
        let pipeline = Pipeline::new(&gpu, &player.cam_uniform);
        let renderer = Renderer::new(pipeline);
        let size = window.inner_size();
        
        // test mesh
        let world = World::new();
        let mesh = mesh_chunk(&world);
        // let mesh = Mesh::default();
        let test_mesh = GPUMesh::from_mesh(&gpu.device, &mesh);
    
        Self {
            window,
            gpu,
            renderer,
            player,
            size,
            test_mesh,
            pressed_keys: HashSet::new(),
            last_update: Instant::now(),
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
    pub fn input(&mut self, event: &Event<()>) -> bool {
        if self.player.input(event, &self.window) {
            return true;
        }
        
        match event {
            Event::WindowEvent { event: window_event, ..} => {
                match window_event {
                    WindowEvent::KeyboardInput { event: key_event, .. } => {
                        if let PhysicalKey::Code(key_code) = key_event.physical_key {
                            match key_event.state {
                                ElementState::Pressed => { self.pressed_keys.insert(key_code); return true}
                                ElementState::Released => { self.pressed_keys.remove(&key_code); return true}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }

    // update gamestate
    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;
        
        self.player.update(&self.pressed_keys, dt);
        
        self.player.cam_uniform = self.player.cam.into_uniform(self.size.width as f32 / self.size.height as f32);
        self.gpu.queue.write_buffer(
            &self.renderer.pipeline.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.player.cam_uniform]),
        );

        self.window.request_redraw();
    }
}