struct Uniform {
    pos:             vec3<f32>,
    width:           u32,
    pixel00_loc:     vec3<f32>,
    height:          u32,
    pixel_delta_w:   vec3<f32>,
    sphere_count:    u32,
    pixel_delta_h:   vec3<f32>,
    quad_count:      u32,
}

struct Sphere {
    center:   vec3<f32>,
    radius:   f32,
    material: Material,
}

struct Quad {
    q:        vec3<f32>,
    _pad0:    f32,
    u:        vec3<f32>,
    _pad1:    f32,
    v:        vec3<f32>,
    d:        f32,
    normal:   vec3<f32>,
    _pad2:    f32,
    material: Material
}

struct HitRecord {
    t:          f32,
    position:   vec3<f32>,
    normal:     vec3<f32>,
    hit:        bool,
    obj_type:   u32, //0 sphere, 1 quad
    obj_index:  u32,
}

struct Material {
    color: vec3<f32>,
    reflect: u32,
    translucent: u32,
}

@group(0) @binding(0) var<uniform>               uniforms:  Uniform;
@group(0) @binding(1) var<storage, read_write>     output:  array<u32>;
@group(0) @binding(2) var<storage, read>          spheres:  array<Sphere>;
@group(0) @binding(3) var<storage, read>            quads:  array<Quad>;


fn ray_at(ray_pos: vec3<f32>, ray_dir: vec3<f32>, t: f32) -> vec3<f32> {
    return ray_pos + t * ray_dir;
}