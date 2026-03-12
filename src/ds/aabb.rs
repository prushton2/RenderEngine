use crate::ds;

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