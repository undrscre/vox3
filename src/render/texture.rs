use std::{fs, path::PathBuf};

use crate::render::device::GPUDevice;

pub struct TextureArray {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl TextureArray {
    pub fn new(
        gpu: &GPUDevice,
        layout: &wgpu::BindGroupLayout,
        images: Vec<PathBuf>, // pass from !include_bytes() or such
        label: &str
    ) -> anyhow::Result<Self> {
        let mut layers = Vec::with_capacity(256);
        let mut width = 0; // no height because. durrr it's a block game there's block textures :clueless:

        for path in &images {
            println!("awawa {:?}", path);
            let file = fs::read(path).expect("unable to read img file");
            let img = image::load_from_memory(&file)?;
            let rgba = img.to_rgba8();
            if width == 0 {
                width = img.width()
            } else if img.width() != width {
                anyhow::bail!("width mismatch found in array, expected {}x{}", width, width);
            }

            layers.extend_from_slice(&rgba.into_raw());
        }

        let amount = images.len() as u32;
        if amount == 0 { anyhow::bail!("no textures found!"); }

        let size = wgpu::Extent3d {
            width: width,
            height: width,
            depth_or_array_layers: amount
        };

        let texture = gpu.device.create_texture(&wgpu::wgt::TextureDescriptor { 
            label: Some(label), 
            size, 
            mip_level_count: 1, 
            sample_count: 1, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Rgba8UnormSrgb, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[]
        });

        gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfoBase { 
                texture: &texture, 
                mip_level: 0, 
                origin: wgpu::Origin3d::ZERO, 
                aspect: wgpu::TextureAspect::All 
            }, 
            &layers,
            wgpu::TexelCopyBufferLayout { 
                offset: 0, 
                bytes_per_row: Some(4 * width), 
                rows_per_image: Some(width) 
            }, 
            size
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(&format!("{} bind group", label)),
        });

        Ok(Self { texture, view, sampler, bind_group })
    }
}