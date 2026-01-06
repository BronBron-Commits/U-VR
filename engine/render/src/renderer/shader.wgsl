struct Camera {
    view_proj : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera : Camera;

struct Model {
    model : mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> model : Model;

struct Player {
    position : vec3<f32>,
    _pad : f32,
};

@group(2) @binding(0)
var<uniform> player : Player;

struct VertexInput {
    @location(0) position : vec3<f32>,
    @location(1) color : vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos : vec4<f32>,
    @location(0) world_pos : vec3<f32>,
    @location(1) base_color : vec3<f32>,
};

@vertex
fn vs_main(input : VertexInput) -> VertexOutput {
    var out : VertexOutput;
    let world = model.model * vec4<f32>(input.position, 1.0);
    out.clip_pos = camera.view_proj * world;
    out.world_pos = world.xyz;
    out.base_color = input.color;
    return out;
}

@fragment
fn fs_main(
    @location(0) world_pos : vec3<f32>,
    @location(1) base_color : vec3<f32>,
) -> @location(0) vec4<f32> {
    let d = distance(world_pos, player.position);

    if (d < 2.5) {
        let t = d * 6.28318;
        let r = 0.5 + 0.5 * sin(t + 0.0);
        let g = 0.5 + 0.5 * sin(t + 2.094);
        let b = 0.5 + 0.5 * sin(t + 4.188);
        return vec4<f32>(r, g, b, 1.0);
    }

    return vec4<f32>(base_color, 1.0);
}
