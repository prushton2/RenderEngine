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

    output[idx] = ray_color(uniforms.pos, ray_dir);
}

// fragment shader is the second pass
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let idx = u32(pos.y) * uniforms.width + u32(pos.x);

    for (var i = 0u; i < uniforms.texture_count; i++) {
        var ui_element = ui_info[i];
        
        var top_left_y = ui_element.v_anchor * ((uniforms.height-ui_element.height)/2);
        var top_left_x = ui_element.h_anchor * ((uniforms.width-ui_element.width)/2);

        // shorthand for this kinda thing
        // switch ui_element.v_anchor {
        //     case 0:
        //         break;
        //     case 1:
        //         top_left_y = uniforms.height/2 - ui_element.height/2;
        //         break;
        //     case 2:
        //         top_left_y = uniforms.height - ui_element.height;
        // }

        if(
            u32(pos.x) >= top_left_x && 
            u32(pos.x) < top_left_x+ui_element.width &&
            u32(pos.y) >= top_left_y &&
            u32(pos.y) < top_left_y+ui_element.height
        ) {
            // we are inbound of the image, so draw the corresponding pixel

            let rel_x = u32(pos.x) - top_left_x;
            let rel_y = u32(pos.y) - top_left_y;

            var c = ui_textures[ui_element.pointer + rel_x + rel_y*ui_element.width];
            let r = f32((c       ) & 0xFFu) / 255.0; // u8s are written as little endian, so i read it backwards
            let g = f32((c >> 8u ) & 0xFFu) / 255.0;
            let b = f32((c >> 16u) & 0xFFu) / 255.0;
            let a = f32((c >> 24u) & 0xFFu) / 255.0;

            c = output[idx];
            let bg = vec4(
                f32((c >> 16u) & 0xFFu) / 255.0,
                f32((c >> 8u)  & 0xFFu) / 255.0,
                f32( c         & 0xFFu) / 255.0,
                1.0
            );

            return vec4(
                r * a + bg.r * (1.0 - a),
                g * a + bg.g * (1.0 - a),
                b * a + bg.b * (1.0 - a),
                1.0
            );
        }
    }

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
