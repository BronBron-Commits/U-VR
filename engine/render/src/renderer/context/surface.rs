use winit::window::Window;
use super::RenderDevice;

pub struct RenderSurface {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
}

impl RenderSurface {
    pub fn new(window: &Window, device: &RenderDevice) -> Self {
        let size = window.inner_size();
        let surface = unsafe { device.instance.create_surface(window) }.unwrap();
        let format = surface.get_capabilities(&device.adapter).formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device.device, &config);

        Self { surface, config }
    }

    pub fn resize(&mut self, width: u32, height: u32, device: &RenderDevice) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&device.device, &self.config);
    }
}
