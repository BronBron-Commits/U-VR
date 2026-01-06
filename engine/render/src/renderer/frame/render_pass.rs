use crate::renderer::context::RenderContext;

pub fn render_frame(ctx: &mut RenderContext) {
    let frame = ctx.surface.surface.get_current_texture().unwrap();
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor { label: None },
    );

    {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
    }

    ctx.device.queue.submit(Some(encoder.finish()));
    frame.present();
}
