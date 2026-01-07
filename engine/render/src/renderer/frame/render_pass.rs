use glam::{Mat3, Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::renderer::context::RenderContext;
use crate::renderer::pipeline::RenderPipelineBundle;
use crate::renderer::resources::mesh::{Mesh, cube_mesh, floor_mesh};
use crate::renderer::uniforms::camera::{CameraUniform, OrbitCamera};
use crate::renderer::Prop;

pub fn render_frame(
    ctx: &mut RenderContext,
    camera: &OrbitCamera,
    avatar_pos: Vec3,
    avatar_yaw: f32,
    props: &[Prop],
) {
    /* ================= FRAME ================= */

    let frame = ctx.surface.surface.get_current_texture().unwrap();
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = ctx.device.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("frame_encoder"),
        },
    );

    /* ================= CAMERA ================= */

    let view_m = camera.view_matrix();
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

    let camera_bind_group_layout =
        ctx.device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
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
        });

    let camera_bind_group =
        ctx.device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

    let pipeline = RenderPipelineBundle::new(
        &ctx.device.device,
        &ctx.surface.config,
        &camera_bind_group_layout,
    );

    /* ================= FLOOR ================= */

    let (fv, fi) = floor_mesh();
    let floor = Mesh::new(&ctx.device.device, &fv, &fi);

    /* ================= AVATAR ================= */

    let (mut av, ai) = cube_mesh([0.9, 0.3, 0.3]); // RED avatar
    let rot = Mat3::from_rotation_y(avatar_yaw);

    for v in &mut av {
        let p = Vec3::from(v.position);
        let rp = rot * (p * Vec3::new(1.0, 2.0, 1.0)) + avatar_pos;
        v.position = rp.into();
    }

    let avatar = Mesh::new(&ctx.device.device, &av, &ai);

    /* ================= PROPS ================= */

    let mut prop_meshes = Vec::new();
    for prop in props {
        let (mut pv, pi) = cube_mesh([0.2, 0.6, 0.9]); // BLUE props
        for v in &mut pv {
            let p = Vec3::from(v.position) * prop.scale + prop.position;
            v.position = p.into();
        }
        prop_meshes.push(Mesh::new(&ctx.device.device, &pv, &pi));
    }

    /* ================= DRAW ================= */

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

        floor.draw(&mut pass);
        avatar.draw(&mut pass);

        for m in &prop_meshes {
            m.draw(&mut pass);
        }
    }

    ctx.device.queue.submit(Some(encoder.finish()));
    frame.present();
}
