struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) dir: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VSOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>( 3.0,  1.0),
        vec2<f32>(-1.0,  1.0),
    );

    let p = positions[i];
    var out: VSOut;
    out.pos = vec4<f32>(p, 0.0, 1.0);
    out.dir = normalize(vec3<f32>(p.x, -p.y, 1.0));
    return out;
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let t = clamp(in.dir.y * 0.5 + 0.5, 0.0, 1.0);

    let horizon = vec3<f32>(0.85, 0.9, 1.0);
    let zenith  = vec3<f32>(0.15, 0.35, 0.65);

    let sky = mix(horizon, zenith, t);

    // subtle sun
    let sun_dir = normalize(vec3<f32>(0.3, 0.7, 0.6));
    let sun = pow(max(dot(in.dir, sun_dir), 0.0), 256.0);

    return vec4<f32>(sky + sun * 1.2, 1.0);
}
