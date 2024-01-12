use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

use crate::vector3::{Triangle, Vector3};

/// Macro that acts like sscanf.
/// https://stackoverflow.com/questions/31046763/does-rust-have-anything-like-scanf
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

/// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
/// Returns a vector of triangles
pub fn load_obj_file(path: String) -> Vec<Triangle> {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        let mut verts: Vec<Vector3> = vec![];
        let mut tris: Vec<Triangle> = vec![];
        for line in lines.flatten() {
            // Very brutish parser, easy to break.
            // TODO: Error handling for erroneous files
            let words = line.split(" ").collect::<Vec<&str>>();
            let command = words[0].chars().nth(0);
            if command == Some('v') {
                let x = words[1].parse().unwrap();
                let y = words[2].parse().unwrap();
                let z = words[3].parse().unwrap();
                verts.push(Vector3::new(x, y, z))
            }
            if command == Some('f') {
                let a: usize = words[1].parse().unwrap();
                let b: usize = words[2].parse().unwrap();
                let c: usize = words[3].parse().unwrap();
                tris.push(Triangle::new(verts[a - 1], verts[b - 1], verts[c - 1]));
            }
        }
        tris
    } else {
        vec![]
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
