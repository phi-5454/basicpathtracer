use rand_distr::num_traits::clamp;

use crate::vector3::*;

// TODO: A filmic tonemapper, ACES perhaps

// An approximate ACES tonemapping.
// From https://64.github.io/tonemapping/#aces
/*
pub fn aces_fit(col: Vector3) -> Vector3 {
    let mut v = col.clone();
    v = 0.6 * v;
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    // The result will be clamped when converted into col.
    (v.star(a * v + b)) / (v.star(c * v + d) + e)
}*/

pub fn reinhard(col: Vector3, max_luminance: f64) -> Vector3 {
    let l_old = luminance(col);
    let numerator = l_old * (1.0 + (l_old / (max_luminance * max_luminance)));
    let l_new = numerator / (1.0 + l_old);
    change_luminance(col, l_new)
}

// From https://64.github.io/tonemapping/
fn luminance(col: Vector3) -> f64 {
    0.2126 * col.x + 0.7152 * col.y + 0.0722 * col.z
}
fn change_luminance(c_in: Vector3, l_out: f64) -> Vector3 {
    let l_in = luminance(c_in);
    return (l_out / l_in) * c_in;
}
