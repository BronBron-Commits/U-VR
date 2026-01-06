use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glam::Vec3;
use render::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("U-VR")
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(Renderer::new(&window));

    let mut input = Vec3::ZERO;
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input: key, .. } => {
                    let pressed = key.state == ElementState::Pressed;

                    if let Some(k) = key.virtual_keycode {
                        match k {
                            VirtualKeyCode::W => input.z = if pressed { -1.0 } else { 0.0 },
                            VirtualKeyCode::S => input.z = if pressed {  1.0 } else { 0.0 },
                            VirtualKeyCode::A => input.x = if pressed { -1.0 } else { 0.0 },
                            VirtualKeyCode::D => input.x = if pressed {  1.0 } else { 0.0 },
                            _ => {}
                        }
                    }
                }
                _ => {}
            },

            Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                let dt = (now - last_frame).as_secs_f32();
                last_frame = now;

                renderer.update(dt, input);
                renderer.render();
            }
            _ => {}
        }
    });
}
