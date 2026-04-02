struct Call {
    ray_pos:   vec3<f32>,
    caller:    i32,
    ray_dir:   vec3<f32>,
    output_id: u32,
    outputs:   array<vec3<f32>, 3>,
    depth:     u32,
    _pad0:     u32,
    _pad1:     u32,
    _pad2:     u32,
}

// var<workgroup> callstack: array<Call, 7>;

fn ray_color(ray_pos: vec3<f32>, ray_dir: vec3<f32>) -> u32 {
    var callstack_len = 1;
    var callstack: array<Call, 8>;

    callstack[0].caller = -1;
    callstack[0].ray_pos = ray_pos;
    callstack[0].ray_dir = ray_dir;
    callstack[0].output_id = 0u;
    callstack[0].outputs[0] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].outputs[1] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].outputs[2] = vec3<f32>(-1.0, -1.0, -1.0);
    callstack[0].depth = 4u;


    while callstack_len != 0 {
        let index = callstack_len-1;
        var call = callstack[index];

        // kill recursion
        if call.depth == 0 {
            callstack[call.caller].outputs[call.output_id] = vec3<f32>(186.0, 219.0, 237.0);
            callstack_len -= 1;
            continue;
        }

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
            // let new_direction = ray.direction - 2.0 * ray.direction.dot(&normal) * normal;

            // let new_ray = ds::Ray::new(&ray.at(t-0.000001), &new_direction);

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

        if !pushed_to_stack {
            call = callstack[index];
            var color = f32(material.translucent) * call.outputs[0] + f32(material.reflect) * call.outputs[1] + f32(100 - material.translucent - material.reflect) * material.color;

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