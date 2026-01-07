struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct Material {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: Material;

struct VSIn {
    @location(0) position: vec3<f32>,
};

struct VSOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(input: VSIn) -> VSOut {
    var out: VSOut;
    out.pos = camera.view_proj * vec4<f32>(input.position, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return material.color;
}
