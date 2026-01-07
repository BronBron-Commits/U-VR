use glam::{Mat3, Vec3};

use crate::renderer::resources::mesh::{Mesh, Vertex};

pub fn draw_compass_overlay(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    device: &wgpu::Device,
    pipeline: &wgpu::RenderPipeline,
    camera_yaw: f32,
) {
    let size = 0.15;
    let origin = Vec3::new(0.85, -0.85, 0.0);

    let rot = Mat3::from_rotation_y(-camera_yaw);

    let axes = [
        (Vec3::X, [1.0, 0.0, 0.0]), // X = red
        (Vec3::Y, [0.0, 1.0, 0.0]), // Y = green
        (Vec3::Z, [0.0, 0.0, 1.0]), // Z = blue
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut i: u16 = 0;

    for (axis, color) in axes {
        let dir = rot * axis * size;

        vertices.push(Vertex {
            position: origin.into(),
            color,
        });
        vertices.push(Vertex {
            position: (origin + dir).into(),
            color,
        });

        indices.push(i);
        indices.push(i + 1);
        i += 2;
    }

    let mesh = Mesh::new(device, &vertices, &indices);

    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("compass_overlay_pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load, // IMPORTANT: do not clear
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    });

    pass.set_pipeline(pipeline);
    mesh.draw(&mut pass);
}
