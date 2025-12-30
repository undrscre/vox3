use cgmath::{Point3, Zero};

use crate::engine::camera::{Camera, CameraUniform};

pub struct Player {
    pub pos: Point3<f32>,
    pub cam: Camera,
    pub cam_uniform: CameraUniform,

    pub speed: f32
}

impl Player {
    pub fn new() -> Self {
        let pos = Point3 {x:0.,y:0.,z:0.};
        let cam = Camera::new(pos);
        let cam_uniform = CameraUniform::new(); // @note: probably decouple this ?? don't know where to though

        Self {
            pos, cam, cam_uniform, speed: 12.
        }
    }
}