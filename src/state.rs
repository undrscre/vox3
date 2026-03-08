use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}, time::Instant};
use cgmath::Point3;
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, WindowEvent}, keyboard::{Key, KeyCode, PhysicalKey}, window::Window};

use crate::{
    engine::player::Player,
    game::{
        chunk::ChunkManager,
        world::{World, WorldConfig, WorldGenerator}
    }, 
    render::{
        device::GPUDevice,
        renderer::Renderer
    }
};

pub struct State {
    pub window: Arc<Window>,
    pub gpu: GPUDevice,
    pub renderer: Renderer,

    pub world_state: WorldState,
    pub input_state: InputState,

    pub player: Player,
    pub last_update:  Instant,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        // actual game shish
        let player = Player::new();
        let gpu = GPUDevice::new(window.clone()).await;
        let renderer = Renderer::new(&gpu);
        
        let world_config = WorldConfig::default();
        info!("using worldconfig {:#?}", world_config);
        let world_state = WorldState::new(world_config);
        let input_state = InputState::new();

        Self {
            window,
            gpu,
            renderer,
            player,
            world_state,
            input_state,
            last_update: Instant::now(),
        }
    }

    // render out the game state
    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render_frame(&self.gpu, &self.player.cam.frustum(self.gpu.size.width as f32 / self.gpu.size.height as f32))
    }

    // gpu surface resize callback
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
                                ElementState::Pressed => { self.input_state.pressed_keys.insert(key_code); return true}
                                ElementState::Released => { self.input_state.pressed_keys.remove(&key_code); return true}
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

    fn tick_deltatime(&mut self) -> f32 {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;
        dt
    }

    // update gamestate
    pub fn update(&mut self) {
        let dt = self.tick_deltatime();

        self.player.update(&self.input_state, &self.world_state, dt);
        self.world_state.update(&self.gpu, &mut self.renderer, self.player.pos);

        self.player.cam_uniform = self.player.cam.into_uniform(self.gpu.size.width as f32 / self.gpu.size.height as f32);
        for stage in &self.renderer.stages {
            stage.update(&self.gpu.queue, self.player.cam_uniform);
        }

        self.window.request_redraw();
    }
}

// helper struct for inputs
pub struct InputState {
    pub pressed_keys: HashSet<KeyCode>,
    pub last_pressed: HashSet<KeyCode>
}

impl InputState {
    pub fn new() -> Self {
        Self { pressed_keys: HashSet::new(), last_pressed: HashSet::new() }
    }
}

// helper struct for world updates
pub struct WorldState {
    pub world: World,
    pub chunk_manager: ChunkManager,
    pub world_generator: Arc<WorldGenerator>,
}

impl WorldState {
    pub fn new(world_config: WorldConfig) -> Self {
        let world_generator = Arc::new(WorldGenerator::new(&world_config));
        let world = World::new(world_config);
        let chunk_manager = ChunkManager::new(20);

        Self { world, chunk_manager, world_generator }
    }

    pub fn update(&mut self, device: &GPUDevice, renderer: &mut Renderer, player_pos: Point3<f32>) {
        self.chunk_manager.update(
            &mut self.world, 
            &self.world_generator, 
            device,
            &mut renderer.resource_manager,
            player_pos
        );
    }
}