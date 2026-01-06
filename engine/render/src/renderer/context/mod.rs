pub mod device;
pub mod surface;

pub use device::RenderDevice;
pub use surface::RenderSurface;

use winit::window::Window;

pub struct RenderContext {
    pub device: RenderDevice,
    pub surface: RenderSurface,
}

impl RenderContext {
    pub async fn new(window: &Window) -> Self {
        let device = RenderDevice::new(window).await;
        let surface = RenderSurface::new(window, &device);
        Self { device, surface }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(width, height, &self.device);
    }
}
