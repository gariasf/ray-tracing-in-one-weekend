#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,


}

impl Interval {
    pub fn new() -> Self {
        Self { min: f64::NEG_INFINITY, max: f64::INFINITY }
    }

    pub fn with_bounds(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}
