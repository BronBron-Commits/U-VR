// ==========================
// Camera (set = 0, binding = 0)
// ==========================
struct Camera {
    view_proj : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera : Camera;


// ==========================
// Material (set = 1, binding = 0)
// ==========================
struct Material {
    color : vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material : Material;


// ==========================
// Vertex IO
// ==========================
struct VertexInput {
    @location(0) position : vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos : vec4<f32>,
};


// ==========================
// Vertex Shader
// ==========================
@vertex
fn vs_main(input : VertexInput) -> VertexOutput {
    var out : VertexOutput;
    out.clip_pos = camera.view_proj * vec4<f32>(input.position, 1.0);
    return out;
}


// ==========================
// Fragment Shader
// ==========================
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return material.color;
}