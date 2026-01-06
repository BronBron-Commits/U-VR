use wgpu::*;
use wgpu::util::DeviceExt;
use winit::window::Window;
use winit::event::VirtualKeyCode;

use crate::world::floor::Floor;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub position: glam::Vec3,
    pub yaw: f32,
    pub pitch: f32,
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

pub struct Renderer {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,

    camera: Camera,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,

    floor: Floor,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(window) }.unwrap();

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
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
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
                buffers: &[crate::world::floor::Vertex::layout()],
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

        let camera = Camera {
            position: glam::vec3(0.0, 2.0, 6.0),
            yaw: -90f32.to_radians(),
            pitch: -0.3,
        };

        let floor = Floor::new(&device);

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            camera,
            camera_buffer,
            camera_bind_group,
            floor,
        }
    }

    pub fn look(&mut self, dx: f32, dy: f32) {
        self.camera.yaw += dx;
        self.camera.pitch = (self.camera.pitch - dy).clamp(-1.5, 1.5);
    }

    pub fn update(&mut self, dt: f32, keys: &std::collections::HashSet<VirtualKeyCode>) {
        let speed = 5.0 * dt;
        let forward = glam::Vec3::new(self.camera.yaw.cos(), 0.0, self.camera.yaw.sin());
        let right = forward.cross(glam::Vec3::Y);

        if keys.contains(&VirtualKeyCode::W) { self.camera.position += forward * speed; }
        if keys.contains(&VirtualKeyCode::S) { self.camera.position -= forward * speed; }
        if keys.contains(&VirtualKeyCode::A) { self.camera.position -= right * speed; }
        if keys.contains(&VirtualKeyCode::D) { self.camera.position += right * speed; }
    }

    pub fn render(&mut self) {
        let aspect = self.config.width as f32 / self.config.height as f32;
        let proj = glam::Mat4::perspective_rh_gl(60f32.to_radians(), aspect, 0.1, 200.0);
        let view = self.camera.view_matrix();

        let uniform = CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
        };

        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&uniform));

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color { r: 0.05, g: 0.05, b: 0.08, a: 1.0 }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.camera_bind_group, &[]);
            self.floor.draw(&mut pass);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
