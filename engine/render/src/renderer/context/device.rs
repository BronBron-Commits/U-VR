use winit::window::Window;

pub struct RenderDevice {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl RenderDevice {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) =
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}
