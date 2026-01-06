use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::renderer::context::RenderContext;
use crate::renderer::pipeline::{RenderPipelineBundle, layouts::BindGroupLayouts};
use crate::renderer::resources::mesh::{Mesh, floor_mesh, cube_mesh};
use crate::renderer::uniforms::camera::CameraUniform;

pub fn render_frame(ctx: &mut RenderContext, avatar_pos: Vec3) {
    let frame = ctx.surface.surface.get_current_texture().unwrap();
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor { label: Some("frame_encoder") },
    );

    // --- Camera ---
    let view_m = Mat4::look_at_rh(
        Vec3::new(0.0, 5.0, 8.0),
        Vec3::ZERO,
        Vec3::Y,
    );

    let proj_m = Mat4::perspective_rh(
        45f32.to_radians(),
        ctx.surface.config.width as f32 / ctx.surface.config.height as f32,
        0.1,
        100.0,
    );

    let cam = CameraUniform {
        view_proj: (proj_m * view_m).to_cols_array_2d(),
    };

    let camera_buffer = ctx.device.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::bytes_of(&cam),
            usage: wgpu::BufferUsages::UNIFORM,
        },
    );

    let layouts = BindGroupLayouts::new(&ctx.device.device);
    let camera_bind_group = ctx.device.device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &layouts.camera,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: None,
        },
    );

    let pipeline = RenderPipelineBundle::new(
        &ctx.device.device,
        &ctx.surface.config,
        &layouts,
    );

    // --- Floor ---
    let (fv, fi) = floor_mesh();
    let floor = Mesh::new(&ctx.device.device, &fv, &fi);

    // --- Avatar cube (CPU translated) ---
    let (mut cv, ci) = cube_mesh();
    for v in &mut cv {
        v.position[0] += avatar_pos.x;
        v.position[1] += avatar_pos.y;
        v.position[2] += avatar_pos.z;
    }
    let cube = Mesh::new(&ctx.device.device, &cv, &ci);

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("main_pass"),
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

        pass.set_pipeline(&pipeline.pipeline);
        pass.set_bind_group(0, &camera_bind_group, &[]);

        // floor
        pass.set_vertex_buffer(0, floor.vertex_buffer.slice(..));
        pass.set_index_buffer(floor.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..floor.index_count, 0, 0..1);

        // avatar cube
        pass.set_vertex_buffer(0, cube.vertex_buffer.slice(..));
        pass.set_index_buffer(cube.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..cube.index_count, 0, 0..1);
    }

    ctx.device.queue.submit(Some(encoder.finish()));
    frame.present();
}
