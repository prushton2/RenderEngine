struct Call {
    ray_pos:   vec3<f32>,
    caller:    i32,
    ray_dir:   vec3<f32>,
    output_id: u32,
    outputs:   array<vec3<f32>, 3>,
    depth:     u32,
}

var<private> callstack: array<Call, 7>;

fn ray_color(ray_pos: vec3<f32>, ray_dir: vec3<f32>, tid: u32) -> u32 {
    var callstack_len = 1;
    let light_dir = vec3<f32>(0.57735,0.57735,0);

    callstack[0].caller = -1;
    callstack[0].ray_pos = ray_pos;
    callstack[0].ray_dir = ray_dir;
    callstack[0].output_id = 0u;
    callstack[0].outputs[0] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].outputs[1] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].outputs[2] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].depth = 4u;

    for (var i = 0; i < 32; i++) {
        if callstack_len == 0 { break; }
        let index = callstack_len-1;
        var call = callstack[index];

        let record = closest_hit(call.ray_pos, call.ray_dir);

        // we missed, so use the sky color
        if !record.hit {
            if call.caller == -1 {
                return 0x00BADBEDu;
            }
            callstack[call.caller].outputs[call.output_id] = vec3<f32>(186.0, 219.0, 237.0);
            callstack_len -= 1;
            continue;
        }
        
        // get the material
        var material: Material;
        var pushed_to_stack = false;

        // kill recursion
        if call.depth == 0 {
            callstack[call.caller].outputs[call.output_id] = material.color;
            callstack_len -= 1;
            continue;
        }

        switch record.obj_type {
            case 0u: {
                material = spheres[record.obj_index].material;
            }
            case 1u: {
                material = quads[record.obj_index].material;
            }
            default: {}
        }

        if material.translucent != 0u && call.outputs[0].x <= -1.0 {
            let new_origin = ray_at(call.ray_pos, call.ray_dir, record.t + 0.00001);
            
            callstack[callstack_len].caller = index;
            callstack[callstack_len].ray_pos = new_origin;
            callstack[callstack_len].ray_dir = call.ray_dir;
            callstack[callstack_len].output_id = 0;
            callstack[callstack_len].outputs[0] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].outputs[1] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].outputs[2] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].depth = call.depth - 1;

            callstack_len += 1;
            pushed_to_stack = true;
        }

        if material.reflect != 0u && call.outputs[1].x <= -1.0 {
            let new_dir = call.ray_dir - (2.0 * dot(call.ray_dir, record.normal) * record.normal);
            let new_origin = ray_at(call.ray_pos, call.ray_dir, record.t - 0.000001);

            callstack[callstack_len].caller = index;
            callstack[callstack_len].ray_pos = new_origin;
            callstack[callstack_len].ray_dir = new_dir;
            callstack[callstack_len].output_id = 1;
            callstack[callstack_len].outputs[0] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].outputs[1] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].outputs[2] = vec3<f32>(-1.0, -1.0, -1.0);
            callstack[callstack_len].depth = call.depth - 1;

            callstack_len += 1;
            pushed_to_stack = true;
        }

        var lit_color = vec3<f32>(0.0, 0.0, 0.0);
        if material.texture_id != -1 {
            lit_color = get_texture_color(record);
        } else {
            lit_color = material.color;
        }

        if material.reflect + material.translucent < 100 {
            let light = max(dot(record.normal, light_dir), 0.0);
            lit_color = vec3<f32>(
                ((lit_color.x + 64.0*light)/2),
                ((lit_color.y + 64.0*light)/2),
                ((lit_color.z + 64.0*light)/2),
            );
        }

        if !pushed_to_stack {
            call = callstack[index];
            var color = f32(material.translucent) * call.outputs[0] + 
                        f32(material.reflect) * call.outputs[1] + 
                        f32(100 - material.translucent - material.reflect) * lit_color;

            color = color / 100.0;

            if call.caller != -1 {
                callstack[call.caller].outputs[call.output_id] = color;
                callstack_len -= 1;
            } else {
                return (u32(color.x) << 16) | (u32(color.y) << 8) | (u32(color.z));
            }
        }
    }

    return 0x00BADBEDu;
}

fn get_texture_color(record: HitRecord) -> vec3<f32> {
    var material: Material;
    var quad: Quad;

    switch record.obj_type {
        case 0u: {
            material = spheres[record.obj_index].material;
            return material.color; // only works on quads for now
        }
        case 1u: {
            material = quads[record.obj_index].material;
            quad = quads[record.obj_index];
        }
        default: {}
    }

    let offset = record.position - quad.q;

    let pct_across = abs(dot(offset, quad.u) / dot(quad.u, quad.u)) * 16.0;
    let pct_up     = abs(dot(offset, quad.v) / dot(quad.v, quad.v)) * 16.0;
    
    return textureLoad(textures, vec2u((15-u32(pct_across)), (15-u32(pct_up)) + 16 * u32(material.texture_id)), 0).rgb * 255.0;
}