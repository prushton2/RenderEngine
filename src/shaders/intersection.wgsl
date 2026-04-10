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

    if t1 > 0.0001 { return t1; }
    if t2 > 0.0001 { return t2; }
    return -1.0;
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
            closest_t     = t;
            rec.hit       = true;
            rec.t         = t;
            rec.position  = origin + t * dir;
            rec.normal    = normalize(rec.position - spheres[i].center);
            rec.obj_type  = 0;
            rec.obj_index = i;
        }
    }

    for (var i = 0u; i < uniforms.quad_count; i++) {
        let t = quad_intersects(origin, dir, quads[i]);
        if t > 0.001 && t < closest_t {
            closest_t     = t;
            rec.hit       = true;
            rec.t         = t;
            rec.position  = origin + t * dir;
            rec.normal    = quads[i].normal;
            rec.obj_type  = 1;
            rec.obj_index = i;
        }
    }

    return rec;
}
