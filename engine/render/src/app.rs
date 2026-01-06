use std::collections::HashSet;
use std::time::Instant;

use glam::Vec3;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::renderer::Renderer;

pub fn run() {
    pollster::block_on(async {
        internal_run().await;
    });
}

async fn internal_run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;

    let mut pressed = HashSet::new();
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                pressed.insert(key);
                            }
                            ElementState::Released => {
                                pressed.remove(&key);
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let dt = (now - last_frame).as_secs_f32();
                last_frame = now;

                let mut movement = Vec3::ZERO;
                let speed = 4.0;

                if pressed.contains(&VirtualKeyCode::W) {
                    movement.z -= speed;
                }
                if pressed.contains(&VirtualKeyCode::S) {
                    movement.z += speed;
                }
                if pressed.contains(&VirtualKeyCode::A) {
                    movement.x -= speed;
                }
                if pressed.contains(&VirtualKeyCode::D) {
                    movement.x += speed;
                }

                renderer.update(dt, movement);
                renderer.render();
            }
            _ => {}
        }
    });
}
