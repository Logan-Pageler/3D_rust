use std::{future::Future, rc::Rc};
use futures::{future::join_all, stream::FuturesUnordered, StreamExt};

use model::{DrawModel, Model};
use resources::{load_model, load_string};
use wgpu::BindGroupLayout;
use winit::{event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};
use cgmath::prelude::*;

pub mod instance;
pub mod model;
pub mod resources;
pub mod texture;

pub struct World {
    pub models: Vec<Model>, 
    is_increase_pressed: bool,
    is_decrease_pressed: bool,
    num_instances: u32,
}

impl World {
    /// Create a new world by loading all possible models and textures
    pub async fn new(device: &Rc<wgpu::Device>, queue: &wgpu::Queue, texture_bind_group_layout: &BindGroupLayout) -> World {
        // we'll use a cube for now

        // load all the models specified in "resources.txt"
        let models = load_string("resources.txt")
            .await
            .unwrap()
            .split("\n")
            .map(|file_name| {
                load_model(file_name, device.clone(), queue, texture_bind_group_layout)
            }).collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Self {
            models,
            is_decrease_pressed: false,
            is_increase_pressed: false,
            num_instances: 0,
        }
    }

    /// handle window events
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    // WASD controls
                    KeyCode::KeyJ => {
                        self.is_increase_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyK => {
                        self.is_decrease_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// update the objects in the world based off the key presses
    pub fn update_world(&mut self) {
        let mut change_occurred = false;
        if self.is_increase_pressed {
            self.num_instances += 1;
            change_occurred = true;
        }
        if self.is_decrease_pressed && self.num_instances > 0{
            self.num_instances -= 1;
            change_occurred = !change_occurred;
        }

        if change_occurred {
            // set up instances
            // this is all our objects
            const SPACE_BETWEEN: f32 = 3.0;

            let num_instances = self.num_instances;

            // we are making a n*n grid of cubes that are rotated at weird angles
            let instances = (0..num_instances).flat_map(|z| {
                (0..num_instances).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - num_instances as f32 / 2.0);
                    let z = SPACE_BETWEEN * (z as f32 - num_instances as f32 / 2.0);

                    let position = cgmath::Vector3 { x, y: 0.0, z };

                    let rotation = if position.is_zero() {
                        cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };

                    instance::Instance {
                        position, rotation,
                    }
                })
            }).collect::<Vec<_>>();

            self.models[0].set_instances(instances);
        }
    }
}

pub trait DrawWorld<'a> {
    fn draw_world(&mut self, world: &'a World, camera_bind_group: &'a wgpu::BindGroup);
}

/// set up drawing models for our RenderPass rendering pipeline
impl<'a, 'b> DrawWorld<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_world(&mut self, world: &'b World, camera_bind_group: &'b wgpu::BindGroup) {
        for model in &world.models {
            self.draw_model(model, camera_bind_group);
        }
    }
}