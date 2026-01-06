use std::collections::HashSet;
use std::time::Instant;

use glam::Vec3;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{CursorGrabMode, WindowBuilder},
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

    let mut jump_requested = false;

    let mut mouse_dx = 0.0f32;
    let mut mouse_dy = 0.0f32;
    let mut scroll = 0.0f32;
    let mut middle_mouse_held = false;

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
                                if key == VirtualKeyCode::Space {
                                    jump_requested = true;
                                }
                                pressed.insert(key);
                            }
                            ElementState::Released => {
                                pressed.remove(&key);
                            }
                        }
                    }
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Middle {
                        middle_mouse_held = state == ElementState::Pressed;

                        if middle_mouse_held {
                            let _ = window.set_cursor_grab(CursorGrabMode::Confined);
                            window.set_cursor_visible(false);
                        } else {
                            let _ = window.set_cursor_grab(CursorGrabMode::None);
                            window.set_cursor_visible(true);
                        }
                    }
                }

                WindowEvent::MouseWheel { delta, .. } => {
                    let amount = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.01,
                    };
                    scroll += amount;
                }

                _ => {}
            },

            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if middle_mouse_held {
                        mouse_dx += delta.0 as f32;
                        mouse_dy += delta.1 as f32;
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
                    input.z -= 1.0;
                }
                if pressed.contains(&VirtualKeyCode::S) {
                    input.z += 1.0;
                }
                if pressed.contains(&VirtualKeyCode::A) {
                    input.x -= 1.0;
                }
                if pressed.contains(&VirtualKeyCode::D) {
                    input.x += 1.0;
                }

                renderer.camera.handle_mouse(
                    mouse_dx,
                    mouse_dy,
                    scroll,
                    middle_mouse_held,
                );

                mouse_dx = 0.0;
                mouse_dy = 0.0;
                scroll = 0.0;

                renderer.update(dt, input, jump_requested);
                jump_requested = false;

                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                renderer.render();
            }

            _ => {}
        }
    });
}
