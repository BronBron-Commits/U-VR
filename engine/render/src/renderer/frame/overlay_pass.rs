use crate::renderer::{
    context::RenderContext,
    pipeline::overlay_pipeline::OverlayPipeline,
    overlay::compass::Compass,
};

pub fn render_overlay(
    ctx: &mut RenderContext,
    view: &wgpu::TextureView,
) {
    let pipeline = OverlayPipeline::new(
        &ctx.device.device,
        ctx.surface.config.format,
    );

    let compass = Compass::new(&ctx.device.device);

    let mut encoder = ctx.device.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("overlay_encoder"),
        },
    );

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("overlay_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&pipeline.pipeline);
        compass.draw(&mut pass);
    }

    ctx.device.queue.submit(Some(encoder.finish()));
}
