use std::fs::File;
use std::io::Write;

mod vector3;
use vector3::*;

mod tonemapper;

const WIDTH: usize = 680;
const HEIGHT: usize = 480;

const MAX_BOUNCES: usize = 3;

///
/// A very basic ray tracer.
/// Loosely based on ssloy's tinyraytracer:
/// https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing
///
fn main() -> std::io::Result<()> {
    render()?;
    Ok(())
}

fn render() -> std::io::Result<()> {
    let mut screen_buffer = vec![255; WIDTH * HEIGHT * 3];

    let sphere = Sphere::new(Vector3::new(0.0, 0.0, -3.0), 1.0);
    let sphere2 = Sphere::new(Vector3::new(-2.0, -1.0, -3.0), 1.0);
    let geom = Renderable {
        material: Material::gray_mat(),
        geometry: Box::new(sphere),
    };
    let geom2 = Renderable {
        material: Material::bluish(),
        geometry: Box::new(sphere2),
    };

    let scene = vec![geom, geom2];

    //let camera: Camera = Camera {
    //pos: Vector3::new(0.0, 0.0, 0.0),
    //dir: Vector3::new(0.0, 1.0, 0.0),
    //fov: (60.0),
    //};

    let mut file = File::create("output.ppm")?;
    println!("{}, ", screen_buffer.len());

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // x and y offsets of the camera directoin
            let xoff = (2.0 * (x as f64 + 0.5) / (WIDTH as f64) - 1.0)
                * (1.57 / 2.0 as f64).tan()
                * (WIDTH as f64)
                / (HEIGHT as f64);
            let yoff =
                -(2.0 * (y as f64 + 0.5) / (HEIGHT as f64) - 1.0) * (1.57 / 2.0 as f64).tan();

            let dir: Vector3 = Vector3::new(xoff, yoff, -1.0).normalize();

            //println!("{}, {}, {}", dir.x, dir.y, dir.z);
            let c = cast_ray(Vector3::new(0.0, 0.0, 0.0), dir, &scene);
            //+ cast_ray(Vector3::new(0.0, 0.0, 0.0), dir, &sphere2);
            write_pixel(x, y, c.as_col(), &mut screen_buffer);
        }
    }

    let output = screen_buffer;
    //let output = screen_buffer.iter().flat_map(|v| [v.x, v.y, v.z]);
    //output.for_each(|a| print!("{}", a));

    write!(file, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;
    file.write_all(&output)?;
    Ok(())
}

fn intersect_scene(
    scene: &Vec<Renderable>,
    origin: Vector3,
    dir: Vector3,
    z0: &mut f64,
) -> Option<Vector3> {
    let mut best = None;
    for g in scene {
        match g.geometry.intersect(origin, dir) {
            Some((z, norm)) => {
                if z < *z0 {
                    *z0 = z;
                    //best = Some(g.material.base_col);
                    best = Some(norm);
                }
            }
            None => {}
        }
    }
    best
}

fn cast_ray(origin: Vector3, dir: Vector3, scene: &Vec<Renderable>) -> Vector3 {
    let mut sphere_dist: f64 = f64::MAX;
    let col = intersect_scene(scene, origin, dir, &mut sphere_dist);
    match col {
        Some(c) => c,
        None => Vector3::new(1.0 * dir.y, 0.0, 0.0),
    }
    /*
    if !gemoetry.geometry.intersect(origin, dir, &mut sphere_dist) {
        Vector3::new(1.0 * dir.y, 0.0, 0.0)
    } else {
        gemoetry.material.base_col
    }*/
}

fn sample_scene(origin: Vector3, dir: Vector3, gemoetry: &Renderable) -> Vector3 {
    let mut dr = dir;
    let mut pos = origin;
    let mut col = Vector3::new(0.0, 0.0, 0.0);
    for n in 0..MAX_BOUNCES {}
    unimplemented!()
}

fn write_pixel(x: usize, y: usize, col: Col, screen_buffer: &mut Vec<u8>) {
    let base_ind = 3 * (x + y * WIDTH);
    screen_buffer[base_ind] = col.r;
    screen_buffer[base_ind + 1] = col.g;
    screen_buffer[base_ind + 2] = col.b;
}

fn ind_to_screenpos(index: usize) -> [usize; 2] {
    let x = index % WIDTH;
    let y = index / HEIGHT;
    [x, y]
}
