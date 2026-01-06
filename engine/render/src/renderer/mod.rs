pub mod camera;
pub mod mesh;
mod instance;

use wgpu::util::DeviceExt;
use glam::{Mat4, Vec3};
use winit::window::Window;

use camera::Camera;
use mesh::{Mesh, Vertex};
use instance::InstanceUniform;
use crate::world::{floor, avatar::Avatar};

pub struct RenderObject {
    pub mesh: Mesh,
    pub instance: InstanceUniform,
    pub instance_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    model_bind_group_layout: wgpu::BindGroupLayout,
    objects: Vec<RenderObject>,
    avatar_index: usize,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let format = surface.get_capabilities(&adapter).formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let camera = Camera::new(config.width as f32, config.height as f32);
        let camera_uniform = camera.uniform();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&camera_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let model_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout, &model_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut objects = Vec::new();

        let (fv, fi) = floor::generate();
        objects.push(Self::make_object(
            &device,
            &model_bind_group_layout,
            fv,
            fi,
            Mat4::IDENTITY,
        ));

        let avatar = Avatar::new();
        let avatar_index = objects.len();
        objects.push(Self::make_object(
            &device,
            &model_bind_group_layout,
            avatar.vertices,
            avatar.indices,
            avatar.model,
        ));

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            camera,
            camera_buffer,
            camera_bind_group,
            model_bind_group_layout,
            objects,
            avatar_index,
        }
    }

    fn make_object(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
        model: Mat4,
    ) -> RenderObject {
        let mesh = Mesh::new(device, &vertices, &indices);
        let instance = InstanceUniform {
            model: model.to_cols_array_2d(),
        };

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&instance),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: instance_buffer.as_entire_binding(),
            }],
            label: None,
        });

        RenderObject {
            mesh,
            instance,
            instance_buffer,
            bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.camera.resize(width as f32, height as f32);
    }

    pub fn update(&mut self, dt: f32, movement: Vec3) {
        let delta = Mat4::from_translation(movement * dt);
        let model =
            Mat4::from_cols_array_2d(&self.objects[self.avatar_index].instance.model);
        let new_model = model * delta;

        self.objects[self.avatar_index].instance.model = new_model.to_cols_array_2d();

        let avatar_pos = new_model.transform_point3(Vec3::ZERO);
        self.camera.follow(avatar_pos);

        self.queue.write_buffer(
            &self.objects[self.avatar_index].instance_buffer,
            0,
            bytemuck::bytes_of(&self.objects[self.avatar_index].instance),
        );

        let camera_uniform = self.camera.uniform();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&camera_uniform),
        );
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.camera_bind_group, &[]);

            for obj in &self.objects {
                pass.set_bind_group(1, &obj.bind_group, &[]);
                pass.set_vertex_buffer(0, obj.mesh.vertex_buffer.slice(..));
                pass.set_index_buffer(
                    obj.mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                pass.draw_indexed(0..obj.mesh.index_count, 0, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
