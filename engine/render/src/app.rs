use std::collections::HashSet;
use std::time::Instant;

use glam::Vec3;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::Renderer;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("U-VR")
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(Renderer::new(&window));

    let mut pressed = HashSet::new();
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

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

                let mut input = Vec3::ZERO;

                if pressed.contains(&VirtualKeyCode::W) {
                    input.z += 1.0;
                }
                if pressed.contains(&VirtualKeyCode::S) {
                    input.z -= 1.0;
                }
                if pressed.contains(&VirtualKeyCode::A) {
                    input.x -= 1.0;
                }
                if pressed.contains(&VirtualKeyCode::D) {
                    input.x += 1.0;
                }

                renderer.update(dt, input);
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                renderer.render();
            }

            _ => {}
        }
    });
}
