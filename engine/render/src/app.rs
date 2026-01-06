use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::Renderer;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("U-VR Client")
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(Renderer::new(&window));

    let mut last_frame = Instant::now();
    let mut keys = std::collections::HashSet::new();
    let mut mouse_pressed = false;
    let mut last_mouse = (0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        renderer.update(dt, &keys);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => { keys.insert(key); }
                            ElementState::Released => { keys.remove(&key); }
                        }
                    }
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Middle {
                        mouse_pressed = state == ElementState::Pressed;
                    }
                }

                WindowEvent::CursorMoved { position, .. } => {
                    if mouse_pressed {
                        let dx = (position.x - last_mouse.0) as f32 * 0.002;
                        let dy = (position.y - last_mouse.1) as f32 * 0.002;
                        renderer.look(dx, dy);
                    }
                    last_mouse = (position.x, position.y);
                }

                _ => {}
            },

            Event::RedrawRequested(_) => {
                renderer.render();
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}
