// definition of most datatypes contained throughout the entire engine

// block definition data
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockTypes {
    AIR,
    STONE
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