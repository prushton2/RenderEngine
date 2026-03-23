struct Uniform {
    pos:           vec3<f32>,
    _pad0:         f32,
    pixel00_loc:   vec3<f32>,
    _pad1:         f32,
    pixel_delta_w: vec3<f32>,
    _pad2:         f32,
    pixel_delta_h: vec3<f32>,
    _pad3:         f32,
    width:         u32,
    height:        u32,
    sphere_count:  u32,
    _pad4:         u32,
    quad_count:    u32,
}

struct Sphere {
    center:   vec3<f32>,
    radius:   f32,
}

struct Quad {
    q:       vec3<f32>,
    _pad0:   f32,
    u:       vec3<f32>,
    _pad1:   f32,
    v:       vec3<f32>,
    d:       f32,
    normal:  vec3<f32>,
    _pad2:   f32,
}

struct HitRecord {
    t:        f32,
    position: vec3<f32>,
    normal:   vec3<f32>,
    hit:      bool,
}

@group(0) @binding(0) var<uniform>               uniforms:  Uniform;
@group(0) @binding(1) var<storage, read_write>     output:  array<u32>;
@group(0) @binding(2) var<storage, read>          spheres:  array<Sphere>;
@group(0) @binding(3) var<storage, read>            quads:  array<Quad>;

fn sphere_intersects(origin: vec3<f32>, dir: vec3<f32>, sphere: Sphere) -> f32 {
    let oc = sphere.center - origin;
    let a  = dot(dir, dir);
    let h  = dot(dir, oc);
    let c  = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        return -1.0;
    }

    let t1 = (h - sqrt(discriminant)) / a;
    let t2 = (h + sqrt(discriminant)) / a;
    return min(t1, t2);
}

fn quad_intersects(origin: vec3<f32>, dir: vec3<f32>, quad: Quad) -> f32 {
    let denominator = dot(quad.normal, dir);

    if abs(denominator) < 0.00000001 {
        return -1.0;
    }

    let t = (quad.d - dot(quad.normal, origin))/denominator;

    let intersection = origin + dir * t;
    let planar_hit = intersection - quad.q;

    let u_len_sq = dot(quad.u, quad.u);
    let v_len_sq = dot(quad.v, quad.v);

    let alpha = dot(quad.u, planar_hit) / u_len_sq;
    let beta = dot(quad.v, planar_hit) / v_len_sq;

    if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
        return -1.0;
    }

    return t;
}

fn closest_hit(origin: vec3<f32>, dir: vec3<f32>) -> HitRecord {
    var rec: HitRecord;
    rec.hit = false;
    var closest_t = 1e30f;

    for (var i = 0u; i < uniforms.sphere_count; i++) {
        let t = sphere_intersects(origin, dir, spheres[i]);
        if t > 0.001 && t < closest_t {
            closest_t    = t;
            rec.hit      = true;
            rec.t        = t;
            rec.position = origin + t * dir;
            rec.normal   = normalize(rec.position - spheres[i].center);
        }
    }

    for (var i = 0u; i < uniforms.quad_count; i++) {
        let t = quad_intersects(origin, dir, quads[i]);
        if t > 0.001 && t < closest_t {
            closest_t    = t;
            rec.hit      = true;
            rec.t        = t;
            rec.position = origin + t * dir;
            rec.normal   = quads[i].normal;
        }
    }

    return rec;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
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

    if uniforms.sphere_count == 0u {
        output[idx] = 0x00FF0000u; // red = sphere_count is 0
        return;
    }
    
    let record = closest_hit(uniforms.pos, ray_dir);

    if record.hit {
        output[idx] = 0x00FF0000;
    } else {
        output[idx] = 0x00BADBED;
    }
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let idx = u32(pos.y) * uniforms.width + u32(pos.x);
    let c = output[idx];
    let r = f32((c >> 16u) & 0xFFu) / 255.0;
    let g = f32((c >> 8u)  & 0xFFu) / 255.0;
    let b = f32( c         & 0xFFu) / 255.0;
    return vec4(r, g, b, 1.0);
}

// mannnnnnnn
@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0),
    );
    return vec4(positions[idx], 0.0, 1.0);
}