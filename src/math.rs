use vecmat::Vector;





pub fn map_range_f64(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn add_scalar(value: Vector<f64, 2>, scalar: f64) -> Vector<f64, 2> {
    value.normalize() * (value.length() + scalar)
} 

pub fn map_range_vec_f64(value: f64, in_min: f64 , in_max: f64, out_min: Vector<f64, 2>, out_max: Vector<f64, 2>) -> Vector<f64, 2> {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}