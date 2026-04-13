@compute @workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_index) local_invocation_index: u32,
) {
    let x = gid.x;
    let y = gid.y;

    if x >= uniforms.width || y >= uniforms.height {
        return;
    }


    let pixel_center = uniforms.pixel00_loc
        + f32(x) * uniforms.pixel_delta_w
        + f32(y) * uniforms.pixel_delta_h;

    let ray_dir = pixel_center - uniforms.pos;

    let idx = y * uniforms.width + x;
    
    if uniforms.sphere_count == 0u && uniforms.quad_count == 0u {
        output[idx] = 0x00FF0000u;
        return;
    }

    output[idx] = ray_color(uniforms.pos, ray_dir);
}

// fragment shader is the second pass
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let idx = u32(pos.y) * uniforms.width + u32(pos.x);
    let c = output[idx];
    let r = f32((c >> 16u) & 0xFFu) / 255.0;
    let g = f32((c >> 8u)  & 0xFFu) / 255.0;
    let b = f32( c         & 0xFFu) / 255.0;
    return vec4(r, g, b, 1.0);
}

// not sure what this does but docs say i need it
@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0),
    );
    return vec4(positions[idx], 0.0, 1.0);
}
