use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}, time::Instant};
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{
    engine::player::Player,
    game::{
        chunk::ChunkManager,
        world::{World, WorldConfig, WorldGenerator}
    }, 
    render::{
        device::GPUDevice, 
        pipeline::{Pipeline, PipelineType}, 
        renderer::Renderer
    }
};

pub struct State {
    pub window: Arc<Window>,
    pub gpu: GPUDevice,

    pub renderer: Renderer,

    pub world: World,
    pub chunk_manager: ChunkManager,
    pub world_generator: Arc<WorldGenerator>,

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

        let mut pipelines: HashMap<PipelineType, Pipeline> = HashMap::new();

        let default_pipeline = Pipeline::default_pipeline(&gpu, &player.cam_uniform);
        pipelines.insert(PipelineType::Default, default_pipeline);
        
        // let debug_pipeline = Pipeline::debug_pipeline(&gpu, &player.cam_uniform);
        // pipelines.insert(PipelineType::DebugWireframe, debug_pipeline);

        let sky_pipeline = Pipeline::sky_pipeline(&gpu, &player.cam_uniform);
        pipelines.insert(PipelineType::Sky, sky_pipeline);

        let renderer = Renderer::new(pipelines);
        
        let world_config = WorldConfig::default();
        info!("using worldconfig {:#?}", world_config);
        let world_generator = Arc::new(WorldGenerator::new(&world_config));
        let world = World::new(world_config);
        let chunk_manager = ChunkManager::new(20);
        
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
        
        self.player.update(&self.pressed_keys, dt, &self.world);

        let player_pos = self.player.pos;
        self.chunk_manager.update(
            &mut self.world, 
            &self.world_generator, 
            &self.gpu,
            &mut self.renderer.resource_manager, 
            player_pos
        );
        
        self.player.cam_uniform = self.player.cam.into_uniform(self.gpu.size.width as f32 / self.gpu.size.height as f32);
        let binding = [self.player.cam_uniform];
        let cam_data = bytemuck::cast_slice(&binding);

        // i gotta write a better api :sobbing:
        if let Some(pipe) = self.renderer.pipelines.get(&PipelineType::Default) {
            self.gpu.queue.write_buffer(&pipe.camera_buffer, 0, cam_data);
        }
        if let Some(pipe) = self.renderer.pipelines.get(&PipelineType::DebugWireframe) {
            self.gpu.queue.write_buffer(&pipe.camera_buffer, 0, cam_data);
        }
        if let Some(pipe) = self.renderer.pipelines.get(&PipelineType::Sky) {
            self.gpu.queue.write_buffer(&pipe.camera_buffer, 0, cam_data);
        }

        self.window.request_redraw();
    }
}