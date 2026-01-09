use std::collections::HashSet;

use cgmath::{InnerSpace, Point3, Vector3};
use winit::{event::{DeviceEvent, Event, MouseButton, WindowEvent}, keyboard::KeyCode};

use crate::engine::camera::{Camera, CameraUniform};

pub struct Player {
    pub pos: Point3<f32>,
    pub cam: Camera,
    pub cam_uniform: CameraUniform,

    pub speed: f32,
    pub sensitivity: f32,
    pub mouse_held: bool,
}

impl Player {
    pub fn new() -> Self {
        let pos = Point3 {x:0.,y:0.,z:0.};
        let cam = Camera::new(pos);
        let cam_uniform = CameraUniform::new(); // @note: probably decouple this ?? don't know where to though
        Self {
            pos, cam, cam_uniform, speed: 12., sensitivity: 0.12, mouse_held: false,
        }
    }

    pub fn update(&mut self, pressed_keys: &HashSet<KeyCode>, dt: f32) {
    
        let mut wish_dir = Vector3::new(0.,0.,0.);
        if pressed_keys.contains(&KeyCode::KeyW) { wish_dir.z += 1.; }
        if pressed_keys.contains(&KeyCode::KeyS) { wish_dir.z -= 1.; }
        if pressed_keys.contains(&KeyCode::KeyA) { wish_dir.x -= 1.; }
        if pressed_keys.contains(&KeyCode::KeyD) { wish_dir.x += 1.; }
        if pressed_keys.contains(&KeyCode::Space) { wish_dir.y += 1.; }
        if pressed_keys.contains(&KeyCode::ShiftLeft) { wish_dir.y -= 1.; }

        if wish_dir != Vector3::new(0., 0., 0.) {
            wish_dir = wish_dir.normalize();

            let forward = Vector3::new(self.cam.forward().x, 0.0, self.cam.forward().z).normalize();
            let right = Vector3::new(self.cam.right().x, 0.0, self.cam.right().z).normalize();
            self.pos += (forward * wish_dir.z + right * wish_dir.x + Vector3::unit_y() * wish_dir.y) * dt * self.speed;
            
            self.cam.pos = self.pos;
            // log::info!("player pos {:?}", self.player.pos);
        }

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