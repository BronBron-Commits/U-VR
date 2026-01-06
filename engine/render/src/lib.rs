use std::time::Instant;

use wgpu::*;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

struct Camera {
    position: glam::Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    fn view_matrix(&self) -> glam::Mat4 {
        let dir = glam::Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        glam::Mat4::look_at_rh(self.position, self.position + dir, glam::Vec3::Y)
    }
}

const SHADER: &str = r#"
struct VSOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
) -> VSOut {
    var out: VSOut;
    out.position = camera.view_proj * vec4<f32>(position, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("U-VR Client")
        .build(&event_loop)
        .unwrap();

    let mut last_frame = Instant::now();
    let mut camera = Camera {
        position: glam::vec3(0.0, 0.0, 2.0),
        yaw: -90.0_f32.to_radians(),
        pitch: 0.0,
    };

    let mut keys = std::collections::HashSet::new();
    let mut mouse_pressed = false;
    let mut last_mouse = (0.0, 0.0);

    let renderer = pollster::block_on(async {
        let size = window.inner_size();
        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let format = surface.get_capabilities(&adapter).formats[0];
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(SHADER.into()),
        });

        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        (surface, device, queue, config, pipeline, vertex_buffer, camera_buffer, camera_bind_group)
    });

    let (surface, device, queue, mut config, pipeline, vertex_buffer, camera_buffer, camera_bind) =
        renderer;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;

        let speed = 3.0 * dt;
        let forward = glam::Vec3::new(camera.yaw.cos(), 0.0, camera.yaw.sin());
        let right = forward.cross(glam::Vec3::Y);

        if keys.contains(&VirtualKeyCode::W) { camera.position += forward * speed; }
        if keys.contains(&VirtualKeyCode::S) { camera.position -= forward * speed; }
        if keys.contains(&VirtualKeyCode::A) { camera.position -= right * speed; }
        if keys.contains(&VirtualKeyCode::D) { camera.position += right * speed; }
        if keys.contains(&VirtualKeyCode::Space) { camera.position.y += speed; }
        if keys.contains(&VirtualKeyCode::LControl) { camera.position.y -= speed; }

        let aspect = config.width as f32 / config.height as f32;
        let proj = glam::Mat4::perspective_rh_gl(60f32.to_radians(), aspect, 0.1, 100.0);
        let view = camera.view_matrix();
        let vp = proj * view;

        let cam_uniform = CameraUniform {
            view_proj: vp.to_cols_array_2d(),
        };
        queue.write_buffer(&camera_buffer, 0, bytemuck::bytes_of(&cam_uniform));

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
        camera.yaw += dx;
        camera.pitch = (camera.pitch - dy).clamp(-1.5, 1.5);
    }
    last_mouse = (position.x, position.y);
}

                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor { label: None });

                {
                    let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color { r: 0.02, g: 0.02, b: 0.04, a: 1.0 }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    pass.set_pipeline(&pipeline);
                    pass.set_bind_group(0, &camera_bind, &[]);
                    pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    pass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}
