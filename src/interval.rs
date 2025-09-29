use std::f64;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

pub const ERROR_CORRECTED_NON_NEGATIVE: Interval = Interval {
    min: 0.001,
    max: f64::INFINITY,
};
