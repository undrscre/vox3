// definition of most datatypes contained throughout the entire engine
// ^ maybe. idk
// decouple everythiong once u got it working

use cgmath::{Point3};

pub const CHUNK_SIZE: usize = 32;

pub type ChunkCoords = i32;

// block definition data
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BlockTypes {
    AIR,
    STONE,
}

// position..whatever
pub fn pack_chunk_coords(x: i32, y: i32, z: i32) -> u64 {
    ((z as u64 & 0x1FFFFF) << 42) |
    ((y as u64 & 0x1FFFFF) << 21) |
    (x as u64 & 0x1FFFFF)
}

pub fn unpack_chunk_coords(packed: u64) -> Point3<ChunkCoords> {
    let x = (packed & 0x1FFFFF) as i32;
    let y = ((packed >> 21) & 0x1FFFFF) as i32;
    let z = ((packed >> 42) & 0x1FFFFF) as i32;

    fn sign_extend(n: i32) -> i32 {
        if (n & 0x100000) != 0 { n | !0x1FFFFF } else { n }
    }

    Point3::new(sign_extend(x), sign_extend(y), sign_extend(z))
}

pub fn world_to_chunk(pos: Point3<f32>) -> Point3<ChunkCoords> {
    Point3::new(
        (pos.x / CHUNK_SIZE as f32).floor() as ChunkCoords,
        (pos.y / CHUNK_SIZE as f32).floor() as ChunkCoords,
        (pos.z / CHUNK_SIZE as f32).floor() as ChunkCoords,
    )
}

// vertex data
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [i32; 3],
    pub packed: u32, // packed data
    // [xxxxxxxx][yyy][ppppppppppppppppppppppp]
    // x - blocktype; y - normals; p - padding
    // ^ this isn't final!!
}

impl Vertex {
    pub const ATTRS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Sint32x3,
        1 => Uint32,
    ];

    // no clue what this means, copied from last iteration of engine
    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}