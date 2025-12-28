use wgpu::{Instance, InstanceDescriptor};
use winit::{dpi::PhysicalSize, window::Window};
use std::sync::Arc;

pub struct GPUDevice {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // pub depth_texture: wgpu::Texture, // maybe decouple these??
    // pub depth_view: wgpu::TextureView,
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
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::defaults(),
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
            present_mode: wgpu::PresentMode::Immediate,
            desired_maximum_frame_latency: 0,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        
        surface.configure(&device, &config);
        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}