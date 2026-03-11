use wgpu::{Instance, InstanceDescriptor};
use winit::{dpi::PhysicalSize, window::Window};
use std::sync::Arc;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

pub struct GPUDevice {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    
    // don't decouple lol it's something global...   maybe
    pub depth_texture: DepthTexture
}

impl GPUDevice {
    // create a gpudevice instance to um. yeah.
    // @todo: you gotta comment better girl
    pub async fn new(window: Arc<Window>) -> Self {
        // boilerplate bullshit .. Please look away !!
        let descriptor = InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        };

        let instance = Instance::new(&descriptor);
        let size = window.inner_size();

        let surface = instance.create_surface(window.clone())
            .expect("dude. fuck. we can't create a surface. it's over");

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::HighPerformance, 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface) 
        }).await.expect("oh!! we're fucked!! Nooo!! adapter died!");
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::POLYGON_MODE_LINE | wgpu::Features::MULTI_DRAW_INDIRECT | wgpu::Features::INDIRECT_FIRST_INSTANCE | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
                required_limits: wgpu::Limits {
                    max_buffer_size: 1024 * 1024 * 1024, 
                    max_storage_buffer_binding_size: 1024 * 1024 * 1024,
                    ..wgpu::Limits::default()
                },
                label: None,
                ..Default::default()
            }
        ).await.unwrap();
        log::info!("Using device: {:#?}", adapter.get_info());
 

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate, // fifo for vsync, immediate for not..?
            desired_maximum_frame_latency: 0,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        
        surface.configure(&device, &config);
        let depth_texture = Self::create_depth_texture(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture
        }
    }

    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> DepthTexture {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::wgt::TextureViewDescriptor::default());
        DepthTexture { texture, view }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            let depth_texture = Self::create_depth_texture(&self.device, &self.config);
            self.depth_texture = depth_texture;
        }
    }
}