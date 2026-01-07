use glam::{Mat3, Vec3};

use crate::renderer::resources::mesh::{Mesh, Vertex};

pub fn draw_compass_overlay(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    device: &wgpu::Device,
    pipeline: &wgpu::RenderPipeline,
    camera_yaw: f32,
) {
    // Length of axis
    let length = 0.18;

    // THICKNESS CONTROL (increase this to make it thicker)
    let thickness = 0.1;

    // Bottom-right of screen in NDC-ish space
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
        let dir = rot * axis * length;

        // Perpendicular vector for thickness (screen-space)
        let perp = Vec3::new(-dir.y, dir.x, 0.0)
            .normalize_or_zero()
            * thickness;

        // Quad corners
        let p0 = origin + perp;
        let p1 = origin - perp;
        let p2 = origin + dir - perp;
        let p3 = origin + dir + perp;

        vertices.push(Vertex { position: p0.into(), color });
        vertices.push(Vertex { position: p1.into(), color });
        vertices.push(Vertex { position: p2.into(), color });
        vertices.push(Vertex { position: p3.into(), color });

        // Two triangles
        indices.extend_from_slice(&[
            i, i + 1, i + 2,
            i, i + 2, i + 3,
        ]);

        i += 4;
    }

    let mesh = Mesh::new(device, &vertices, &indices);

    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("compass_overlay_pass"),
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

    pass.set_pipeline(pipeline);
    mesh.draw(&mut pass);
}
