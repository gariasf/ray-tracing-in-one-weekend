use rand::Rng;

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float() -> f64 {
    return  rand::thread_rng().gen_range(0.0..1.0);
}

pub fn random_float_range(min: f64, max: f64) -> f64 {
    return  rand::thread_rng().gen_range(min..max);
}