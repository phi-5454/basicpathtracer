use std::fs::File;
use std::io::Write;

use std::thread;

mod vector3;
use objloader::load_obj_file;
use vector3::*;

mod objloader;

//mod tonemapper;

const WIDTH: usize = 680;
const HEIGHT: usize = 480;

const MAX_BOUNCES: usize = 1;
const MAX_SAMPLES: usize = 1;

///
/// A very basic ray tracer.
/// Loosely based on ssloy's tinyraytracer:
/// https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing
///
/// TODO: Time it
fn main() -> std::io::Result<()> {
    render()?;
    Ok(())
}

fn render() -> std::io::Result<()> {
    // TODO: handle errors
    // buffer against thread hiccups
    let par = thread::available_parallelism().unwrap().get() * 4;
    let pixels = WIDTH * HEIGHT;
    // Generate pixel ranges for the threads
    let ranges = (0..par)
        .into_iter()
        .map(|t| ((t * pixels) / par)..(((t + 1) * pixels) / par));
    /*ranges.for_each(|r| {
        let min = r.clone().min();
        let max = r.max();
        println!("{}..{}", &min.unwrap(), max.unwrap())
    });*/
    println!("Altogether {} (-1)", pixels);

    let mut screen_buffer = vec![255; WIDTH * HEIGHT * 3];

    let mut file = File::create("output.ppm")?;
    println!("{}, ", screen_buffer.len());

    let a: Vec<_> = ranges
        .map(move |t| {
            println!("SPAWNING");
            thread::spawn(move || {
                //println!("{}, {}", t.clone().min().unwrap(), t.clone().max().unwrap());
                (t.clone(), render_partial(&t))
            })
        })
        .collect();

    a.into_iter().for_each(|t| {
        let a = t.join().expect("Some thread failed");
        for i in a.0.clone() {
            write_pixel_ind(i, a.1[i - a.0.clone().min().unwrap()], &mut screen_buffer)
        }
        println!("Joined: (something)");
    });
    println!("par: {}", par);
    let r = 0..pixels;

    //let camera: Camera = Camera {
    //pos: Vector3::new(0.0, 0.0, 0.0),
    //dir: Vector3::new(0.0, 1.0, 0.0),
    //fov: (60.0),
    //};

    /*
    for y in 0..HEIGHT {
        for x in 0..WIDTH {}
    }*/

    let output = screen_buffer;
    //let output = screen_buffer.iter().flat_map(|v| [v.x, v.y, v.z]);
    //output.for_each(|a| print!("{}", a));

    write!(file, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;
    file.write_all(&output)?;
    Ok(())
}

// Render a part of the image
fn render_partial(pixel_indices: &std::ops::Range<usize>) -> Vec<Col> {
    // This should not be here. Just a hack until I figure out sharing between threads
    let sphere = Sphere::new(Vector3::new(0.0, 0.0, -3.0), 1.0);
    let sphere2 = Sphere::new(Vector3::new(-150.0, 100.0, -200.0), 50.0);
    let sphere3 = Sphere::new(Vector3::new(1.8, -1.0, -2.0), 0.22);
    let sphere4 = Sphere::new(Vector3::new(2.0, 0.0, -3.0), 1.0);
    let sphere5 = Sphere::new(Vector3::new(0.0, -41.0, -3.0), 40.0);
    let sphere6 = Sphere::new(Vector3::new(-1.5, -0.3, -2.5), 0.8);
    let geom = Renderable {
        material: Material::gray_mat(),
        geometry: Box::new(sphere),
    };
    let geom2 = Renderable {
        material: Material::white_light(),
        geometry: Box::new(sphere2),
    };
    let geom3 = Renderable {
        material: Material::bluish(),
        geometry: Box::new(sphere3),
    };
    let geom4 = Renderable {
        material: Material::gray_mat(),
        geometry: Box::new(sphere4),
    };
    let geom5 = Renderable {
        material: Material::gray_mat(),
        geometry: Box::new(sphere5),
    };
    let geom6 = Renderable {
        material: Material::semirough(),
        geometry: Box::new(sphere6),
    };
    let geom_bunny = Renderable {
        material: Material::semirough(),
        geometry: Box::new(Mesh::from(
            "./bunny.obj".to_string(),
            Vector3::new(0.0, 0.0, -1.0),
            3.0,
        )),
    };

    let scene = vec![geom, geom2, geom3, geom4, geom5, geom6, geom_bunny];

    let mut ret_vec = vec![
        Col {
            r: 255,
            g: 255,
            b: 255
        };
        pixel_indices.len()
    ];
    for i in pixel_indices.clone() {
        let x = i % WIDTH;
        let y = i / WIDTH;

        // x and y offsets of the camera direction
        let xoff = (2.0 * (x as f64 + 0.5) / (WIDTH as f64) - 1.0)
            * (1.57 / 2.0 as f64).tan()
            * (WIDTH as f64)
            / (HEIGHT as f64);
        let yoff = -(2.0 * (y as f64 + 0.5) / (HEIGHT as f64) - 1.0) * (1.57 / 2.0 as f64).tan();

        let dir: Vector3 = Vector3::new(xoff, yoff, -1.0).normalize();

        //println!("{}, {}, {}", dir.x, dir.y, dir.z);
        let c = gather_scene(Vector3::new(0.0, 0.0, 0.0), dir, &scene);
        //+ cast_ray(Vector3::new(0.0, 0.0, 0.0), dir, &sphere2);
        ret_vec[i - pixel_indices.clone().min().unwrap()] = c.as_col();
        //        write_pixel_ind(i, c.as_col(), &mut ret_vec);
    }
    ret_vec
}

/// returns: on hit: (hit material, hit normal)
///
fn intersect_scene(
    scene: &Vec<Renderable>,
    origin: Vector3,
    dir: Vector3,
    z0: &mut f64,
) -> Option<(Material, Vector3)> {
    let mut best = None;
    for g in scene {
        match g.geometry.intersect(origin, dir) {
            Some((z, norm)) => {
                if z < *z0 {
                    *z0 = z;
                    //best = Some(g.material.base_col);
                    best = Some((g.material.clone(), norm));
                }
            }
            None => {}
        }
    }
    best
}

/// colcumulative is what we are looking for, in terms of render color
/// Returns: ("cumulative color", "color factor", (new position, new direction)?) newdir none if no bounce.
fn cast_ray(
    origin: Vector3,
    dir: Vector3,
    scene: &Vec<Renderable>,
    col_cum: Vector3,
    col_factor: Vector3,
) -> (Vector3, Vector3, Option<(Vector3, Vector3)>) {
    let mut dist: f64 = f64::MAX;
    let intr = intersect_scene(scene, origin, dir, &mut dist);
    match intr {
        Some((mat, norm)) => {
            let newpos = dist * dir + origin;
            // TODO: Extract color calculation to separate function
            let newdir = new_dir(dir, norm, &mat);
            // emmissive contribution.
            // Take lambertian falloff into account
            let cumcol = col_cum + mat.emissive.star(col_factor);
            // effect of surface color
            let tot_factor = col_factor.star(mat.base_col);
            (cumcol, tot_factor, Some((newpos, newdir)))
        }
        None => {
            // Clear color acts as emissive does.
            // TODO: Extract calculation into inline function
            let cumcol = col_cum + clear_col(dir).star(col_factor);
            (cumcol, col_factor, None)
        }
    }
    /*
    if !gemoetry.geometry.intersect(origin, dir, &mut sphere_dist) {
        Vector3::new(1.0 * dir.y, 0.0, 0.0)
    } else {
        gemoetry.material.base_col
    }*/
}

fn clear_col(dir: Vector3) -> Vector3 {
    Vector3::new(0.2, 1.0 * (-dir.y).max(0.0), 1.0 * (dir.y).max(0.0))
}

// Calculate direction of bounced ray
fn new_dir(d: Vector3, n: Vector3, mat: &Material) -> Vector3 {
    let res = Vector3::on_unit_sphere();
    //let diff = if (res * n) >= 0.0 { (res + n) } else { -1.0 * res };
    let diff = (res + n).normalize();

    let spec = d - 2.0 * (d * n) * n;
    Vector3::lerp(spec, diff, mat.roughness)
    // Currently, just the reflection vector
    //n
}

// TODO: sample_scene and cast_ray could be restructured
fn sample_scene(origin: Vector3, dir: Vector3, scene: &Vec<Renderable>) -> Vector3 {
    let mut orig = origin;
    let mut cum_col = Vector3::new(0.0, 0.0, 0.0);
    let mut col_factor = Vector3::new(1.0, 1.0, 1.0);
    let mut dir = dir;

    for n in (0..=MAX_BOUNCES).rev() {
        let res = cast_ray(orig, dir, scene, cum_col, col_factor);
        (cum_col, col_factor) = (res.0, res.1);
        match res.2 {
            Some((p, d)) => {
                orig = p + (0.001 * d); // prevent self-intersection
                dir = d
            }
            None => break,
        }
    }
    cum_col
}

// The sample and gather
fn gather_scene(origin: Vector3, dir: Vector3, scene: &Vec<Renderable>) -> Vector3 {
    let mut res = Vector3::new(0.0, 0.0, 0.0);
    for n in 1..=MAX_SAMPLES {
        let col = sample_scene(
            origin,
            (dir + 0.001 * Vector3::on_unit_sphere()).normalize(),
            scene,
        );
        let frac = 1.0 / (n as f64);
        res = frac * col + (1.0 - frac) * res
    }
    res
}

fn write_pixel(x: usize, y: usize, col: Col, screen_buffer: &mut Vec<u8>) {
    let base_ind = 3 * (x + y * WIDTH);
    screen_buffer[base_ind] = col.r;
    screen_buffer[base_ind + 1] = col.g;
    screen_buffer[base_ind + 2] = col.b;
}
fn write_pixel_ind(i: usize, col: Col, buffer: &mut Vec<u8>) {
    let base_ind = 3 * i;
    buffer[base_ind] = col.r;
    buffer[base_ind + 1] = col.g;
    buffer[base_ind + 2] = col.b;
}

fn ind_to_screenpos(index: usize) -> [usize; 2] {
    let x = index % WIDTH;
    let y = index / HEIGHT;
    [x, y]
}
