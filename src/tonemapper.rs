use crate::vector3::*;

// TODO: A filmic tonemapper, ACES perhaps

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
