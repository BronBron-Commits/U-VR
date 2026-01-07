use glam::{Mat3, Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::renderer::context::RenderContext;
use crate::renderer::pipeline::RenderPipelineBundle;
use crate::renderer::resources::mesh::{Mesh, floor_mesh, cube_mesh};
use crate::renderer::uniforms::camera::{CameraUniform, OrbitCamera};
use crate::renderer::frame::overlay_pass::draw_compass_overlay;
use crate::renderer::skybox::skybox_pass::draw_skybox;
use crate::renderer::skybox::skybox_pipeline::SkyboxPipeline;
use crate::renderer::Prop;

pub fn render_frame(
    ctx: &mut RenderContext,
    camera: &OrbitCamera,
    avatar_pos: Vec3,
    avatar_yaw: f32,
    props: &[Prop],                 // ← USED AGAIN
    pipelines: &RenderPipelineBundle,
    skybox: &SkyboxPipeline,
) {
    let frame = ctx.surface.surface.get_current_texture().unwrap();
    let view_tex = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("frame_encoder"),
        },
    );

    /* ================= SKYBOX PASS ================= */

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("skybox_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view_tex,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        draw_skybox(&mut pass, skybox);
    }

    /* ================= CAMERA ================= */

    let view_m = camera.view_matrix();
    let proj_m = Mat4::perspective_rh(
        45.0_f32.to_radians(),
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

    let camera_bind_group = ctx.device.device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &ctx.camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: None,
        },
    );

    /* ================= FLOOR ================= */

    let (fv, fi) = floor_mesh();
    let floor = Mesh::new(&ctx.device.device, &fv, &fi);

    /* ================= AVATAR PARTS ================= */

    let mut avatar_meshes = Vec::new();

    for prop in props {
        let (mut verts, inds) = cube_mesh([0.8, 0.8, 0.8]);

        for v in &mut verts {
            let p = Vec3::from(v.position) * prop.scale + prop.position;
            v.position = p.into();
        }

        avatar_meshes.push(Mesh::new(&ctx.device.device, &verts, &inds));
    }

    /* ================= WORLD PASS ================= */

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("world_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view_tex,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &ctx.depth.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        pass.set_pipeline(&pipelines.main);
        pass.set_bind_group(0, &camera_bind_group, &[]);

        floor.draw(&mut pass);

        for mesh in &avatar_meshes {
            mesh.draw(&mut pass);
        }
    }

    /* ================= COMPASS ================= */

    draw_compass_overlay(
        &mut encoder,
        &view_tex,
        &ctx.device.device,
        &pipelines.overlay,
        camera.yaw,
    );

    ctx.device.queue.submit(Some(encoder.finish()));
    frame.present();
}
