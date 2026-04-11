use crate::{ds, object::Renderable};

pub struct Aabb { // Axis aligned bounding box
    x: ds::Interval,
    y: ds::Interval,
    z: ds::Interval
}

impl Aabb {
    pub fn from_interval(x: ds::Interval, y: ds::Interval, z: ds::Interval) -> Self {
        let mut this = Self {
            x: x,
            y: y,
            z: z
        };
        
        this.pad_to_min();
        this
    }

    pub fn from_vector3(pointa: &ds::Vector3, pointb: &ds::Vector3) -> Self {
        let mut this = Self {
            x: ds::Interval::new(pointa.x, pointb.x),
            y: ds::Interval::new(pointa.y, pointb.y),
            z: ds::Interval::new(pointa.z, pointb.z),
        };

        this.pad_to_min();
        this
    }

    pub fn from_aabb(box1: Self, box2: Self) -> Self {
        Self::from_interval(
            ds::Interval::merge_intervals(box1.x, box2.x),
            ds::Interval::merge_intervals(box1.y, box2.y),
            ds::Interval::merge_intervals(box1.z, box2.z),
        )
    }

    fn pad_to_min(&mut self) {
        let delta: f64 = 0.0001;

        if self.x.size() < delta { self.x.expand(delta); }
        if self.y.size() < delta { self.y.expand(delta); }
        if self.z.size() < delta { self.z.expand(delta); }
    }
}

impl Renderable for Aabb {
    fn intersects(&self, ray: &ds::Ray) -> bool {
        let axes = [ // each slab we check
            (&self.x, ray.origin.x, ray.direction.x),
            (&self.y, ray.origin.y, ray.direction.y),
            (&self.z, ray.origin.z, ray.direction.z),
        ];

        let mut t_enter = f64::NEG_INFINITY;
        let mut t_exit  = f64::INFINITY;

        for (interval, origin, dir) in axes {
            if dir.abs() < f64::EPSILON {
                // Ray is parallel to this slab — check if origin is inside it
                if origin < interval.min() || origin > interval.max() {
                    return false;
                }
                continue;
            }

            // between the min and max of the slab
            let t0 = (interval.min() - origin) / dir;
            let t1 = (interval.max() - origin) / dir;

            let (t_near, t_far) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            t_enter = t_enter.max(t_near);
            t_exit  = t_exit.min(t_far);

            if t_enter > t_exit {
                return false;
            }
        }
        return true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn center(&self) -> crate::ds::Vector3 {
        ds::Vector3::new(self.x.middle(), self.y.middle(), self.z.middle())
    }
}