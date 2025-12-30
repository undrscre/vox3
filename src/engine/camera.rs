use cgmath::{Deg, InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3, perspective};

pub struct Camera {
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    
    pub sensitivity: f32,

    pub pos: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

const OPENGL_TO_WGPU: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Default for Camera {
    fn default() -> Self {
        Self {
            fov: 90.0,
            znear: 0.01,
            zfar: 1000.,
            sensitivity: 0.12,
            pos: Point3::new(0., 0., 0.),
            yaw: -90.0,
            pitch: 0.0
        }
    }
}

impl Camera {
    pub fn new(pos: Point3<f32>) -> Self {
        Self {pos, ..Default::default()}
    }

     pub fn forward(&self) -> Vector3<f32> {
        let yaw_r = Rad(self.yaw.to_radians());
        let pitch_r = Rad(self.pitch.to_radians());

        Vector3 {
            x: yaw_r.0.cos() * pitch_r.0.cos(),
            y: pitch_r.0.sin(),
            z: -yaw_r.0.sin() * pitch_r.0.cos(),
        }
        .normalize()
    }

    pub fn right(&self) -> Vector3<f32> {
        self.forward().cross(Vector3::unit_y()).normalize()
    }

    pub fn up(&self) -> Vector3<f32> {
        self.right().cross(self.forward()).normalize()
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            self.pos, 
            self.pos + self.forward(), 
            Vector3::unit_y()
        )
    }

    pub fn proj_matrix(&self, aspect: f32) -> Matrix4<f32> {
        perspective(Deg(self.fov), aspect, self.znear, self.zfar)
    }

    pub fn add_yaw_pitch(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-89.0, 89.0);
    }

    pub fn into_uniform(&self, aspect: f32) -> CameraUniform {
        CameraUniform { view_proj: (OPENGL_TO_WGPU * self.proj_matrix(aspect) * self.view_matrix()).into() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { view_proj: Matrix4::identity().into() }
    }
}