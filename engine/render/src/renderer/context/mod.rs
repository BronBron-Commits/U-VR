use winit::window::Window;

pub mod device;
pub mod surface;
pub mod depth;

use crate::renderer::context::device::RenderDevice;
use crate::renderer::context::surface::RenderSurface;
use crate::renderer::context::depth::DepthTexture;

pub struct RenderContext {
    pub device: RenderDevice,
    pub surface: RenderSurface,
    pub camera_layout: wgpu::BindGroupLayout,
    pub depth: DepthTexture,
}

impl RenderContext {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("failed to find adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("failed to create device");

        let render_device = RenderDevice {
            instance,
            adapter,
            device,
            queue,
        };

        let surface = RenderSurface::new(window, &render_device);
        let depth = DepthTexture::new(&render_device.device, &surface.config);

        let camera_layout =
            render_device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("camera_bind_group_layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        Self {
            device: render_device,
            surface,
            camera_layout,
            depth,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(width, height, &self.device);
        self.depth = DepthTexture::new(&self.device.device, &self.surface.config);

    }
}
