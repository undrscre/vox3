use std::{collections::HashSet, mem, sync::Arc, time::Instant};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{
    engine::{data::pack_chunk_coords, player::Player},
    game::{
        chunk::ChunkManager,
        world::{World, WorldConfig, WorldGenerator}
    }, 
    render::{
        device::GPUDevice, 
        pipeline::Pipeline, 
        renderer::Renderer
    }
};

pub struct State {
    pub window: Arc<Window>,
    pub gpu: GPUDevice,

    pub renderer: Renderer,

    pub world: World,
    pub chunk_manager: ChunkManager,
    pub world_generator: WorldGenerator,

    // todo consolidate to another struct
    pub player: Player,
    pub pressed_keys: HashSet<KeyCode>,
    pub last_update:  Instant,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        // actual game shish
        let player = Player::new();

        let gpu = GPUDevice::new(window.clone()).await;
        let pipeline = Pipeline::new(&gpu, &player.cam_uniform);
        let renderer = Renderer::new(pipeline);
        
        let world_config = WorldConfig::default();
        let world = World::new(world_config);
        let chunk_manager = ChunkManager::new(8);
        let world_generator = WorldGenerator::new();
        
        Self {
            window,
            gpu,
            renderer,
            player,
            pressed_keys: HashSet::new(),
            last_update: Instant::now(),

            world,
            chunk_manager,
            world_generator
        }
    }

    // render out the game state
    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render_frame(&self.gpu)
    }

    // kjlsfdgkjlsfdakl
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
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

        let player_pos = self.player.pos;
        let chunk_commands = self.chunk_manager.get_load_commands(player_pos);
        for cmd in chunk_commands {
            let key = pack_chunk_coords(cmd.x, cmd.y, cmd.z);
            if !self.world.chunks.contains_key(&key) {
                let new_chunk = self.world_generator.generate(&self.world.metadata, cmd);
                self.world.insert(cmd, new_chunk);
            }
        }

        self.renderer.sync_world(&self.gpu, &mut self.world);

        self.player.cam_uniform = self.player.cam.into_uniform(self.gpu.size.width as f32 / self.gpu.size.height as f32);
        self.gpu.queue.write_buffer(
            &self.renderer.pipeline.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.player.cam_uniform]),
        );

        self.window.request_redraw();
    }
}