pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min: min,
            max: max
        }
    }

    pub fn merge_intervals(a: Self, b: Self) -> Self {
        Self {
            min: if a.min < b.min { a.min } else { b.min },
            max: if a.max > b.max { a.max } else { b.max }
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY
        }
    }

    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn expand(&mut self, size: f64) {
        self.min -= size/2.0;
        self.max += size/2.0;
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn middle(&self) -> f64 {
        (self.max - self.min)/2.0 + self.min
    }
}