use std::fs::File;
use std::io::Write;

mod vector3;
use vector3::Col;
use vector3::Vector3;

const WIDTH: usize = 680;
const HEIGHT: usize = 480;

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

    let a = 5 * Vector3::new(5, 2, 1);
    println!("{}, {}, {}", a.x, a.y, a.z);
    let b = Vector3::new(5, 2, 1) + Vector3::new(5, 2, 1);
    println!("{}, {}, {}", b.x, b.y, b.z);
    let c = Vector3::new(5, 2, 1) - Vector3::new(5, 2, 1);
    println!("{}, {}, {}", c.x, c.y, c.z);

    let mut file = File::create("output.ppm")?;
    println!("{}, ", screen_buffer.len());

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let colcheck = collision_check([x, y]);
            let c = if colcheck {
                Col {
                    r: 50,
                    g: 50,
                    b: 50,
                }
            } else {
                Col { r: 0, g: 0, b: 0 }
            };
            write_pixel(x, y, c, &mut screen_buffer)
        }
    }

    let output = screen_buffer;
    //let output = screen_buffer.iter().flat_map(|v| [v.x, v.y, v.z]);
    //output.for_each(|a| print!("{}", a));

    writeln!(file, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;
    file.write_all(&output)?;
    Ok(())
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

fn collision_check(screen_pos: [usize; 2]) -> bool {
    let x: i32 = screen_pos[0].try_into().unwrap();
    let y: i32 = screen_pos[1].try_into().unwrap();
    let midx: i32 = (WIDTH / 2).try_into().unwrap();
    let midy: i32 = (HEIGHT / 2).try_into().unwrap();
    match (midx - x).pow(2) + (midy - y).pow(2) < 10000 {
        true => true,
        false => false,
    }
}
