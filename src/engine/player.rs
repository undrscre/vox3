use cgmath::{InnerSpace, Point3, Vector3, Zero};
use winit::{event::{DeviceEvent, Event, MouseButton, WindowEvent}, keyboard::KeyCode};

use crate::{engine::{camera::{Camera, CameraUniform}, data::{BlockId}}, game::{world::World}, state::{InputState, WorldState}};

pub struct Player {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,

    pub on_ground: bool,

    pub cam: Camera,
    pub cam_uniform: CameraUniform,

    pub speed: f32,
    pub sensitivity: f32,
    pub mouse_held: bool,
}

impl Player {
    pub fn new() -> Self {
        let pos = Point3 {x:0.,y:300.,z:0.};
        let cam = Camera::new(pos);
        let cam_uniform = CameraUniform::new(); // @note: probably decouple this ?? don't know where to though
        Self {
            pos, 
            vel: Vector3::zero(),
            on_ground: false,

            cam, 
            cam_uniform,

            speed: 12., sensitivity: 0.12, mouse_held: false,
        }
    }

    pub fn update(&mut self, input: &InputState, world_state: &WorldState, dt: f32) {
        let mut wish_dir = Vector3::new(0.,0.,0.);
        if input.pressed_keys.contains(&KeyCode::KeyW) { wish_dir.z += 1.; }
        if input.pressed_keys.contains(&KeyCode::KeyS) { wish_dir.z -= 1.; }
        if input.pressed_keys.contains(&KeyCode::KeyA) { wish_dir.x -= 1.; }
        if input.pressed_keys.contains(&KeyCode::KeyD) { wish_dir.x += 1.; }

        // gravity constant
        self.vel.y -= 35.0 * dt;

        if self.on_ground && input.pressed_keys.contains(&KeyCode::Space) {
            self.vel.y = 10.0;
            self.on_ground = false;
        }

        if wish_dir.magnitude() > 0.0 {
            wish_dir = wish_dir.normalize();
            let forward = Vector3::new(self.cam.forward().x, 0.0, self.cam.forward().z).normalize();
            let right = Vector3::new(self.cam.right().x, 0.0, self.cam.right().z).normalize();
            
            let horizontal_move = (forward * wish_dir.z + right * wish_dir.x) * self.speed;
            self.vel.x = horizontal_move.x;
            self.vel.z = horizontal_move.z;
        } else {
            self.vel.x = 0.0;
            self.vel.z = 0.0;
        }

        self.on_ground = false;

        // oh goddd
        let next_y = self.pos.y + self.vel.y * dt;
        if !self.is_colliding(Point3::new(self.pos.x, next_y, self.pos.z), &world_state.world) {
            self.pos.y = next_y;
        } else {
            if self.vel.y < 0.0 { 
                self.on_ground = true;
                // self.pos.y = next_y.ceil(); 
            }
            self.vel.y = 0.0;
        }

        let next_x = self.pos.x + self.vel.x * dt;
        let next_z = self.pos.z + self.vel.z * dt;

        if !self.is_colliding(Point3::new(next_x, self.pos.y, self.pos.z), &world_state.world) {
            self.pos.x = next_x;
        } else {
            self.vel.x = 0.0;
        }

        if !self.is_colliding(Point3::new(self.pos.x, self.pos.y, next_z), &world_state.world) {
            self.pos.z = next_z;
        } else {
            self.vel.z = 0.0;
        }

        self.cam.pos = self.pos;
    }

    pub fn is_colliding(&self, next_position: Point3<f32>, world: &World) -> bool {
        let radius = 0.29;
        let height = 1.79;

        for dx in [-radius, radius] {
            for dy in [-height, -(height / 2.0), 0.0] {
                for dz in [-radius, radius] {
                    let check_pos = next_position + Vector3::new(dx, dy, dz);
                    if world.get_block_at(check_pos) != BlockId::AIR {
                        return true;
                    }
                }
            }
        }

        false
    }

    // capture input
    pub fn input(&mut self, event: &Event<()>, window: &winit::window::Window) -> bool {
        match event {
            Event::WindowEvent { event: window_event, .. } => {
                match window_event {
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Right { 
                            if state.is_pressed() {
                                window.set_cursor_visible(false);
                                window.set_cursor_grab(winit::window::CursorGrabMode::Confined).unwrap();
                                self.mouse_held = true
                            } else {
                                window.set_cursor_visible(true);
                                window.set_cursor_grab(winit::window::CursorGrabMode::None).unwrap();
                                self.mouse_held = false
                            }
                        }
                    }
                    _ => {}
                }
            },
            
            Event::DeviceEvent { event: device_event, .. } => {
                match device_event {
                    DeviceEvent::MouseMotion { delta } => {
                        if !self.mouse_held {return false};
                        self.cam.add_yaw_pitch(-delta.0 as f32 * self.sensitivity, -delta.1 as f32 * self.sensitivity);
                    }
                    _ => {}
                }
            },
            _ => {}
        }
        false
    }
}