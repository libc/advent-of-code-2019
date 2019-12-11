use std::fs::File;
use std::io::prelude::*;

fn load_file(fname: &str) -> std::io::Result<Vec<(f64, f64)>> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut output = Vec::new();

    for (y, line) in contents.trim().split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                output.push((x as f64, y as f64));
            }
        }
    }

    Ok(output)
}

fn solve(fname: &str) -> std::io::Result<(usize, f64, f64)> {
    let asteroids = load_file(fname)?;
    let mut max = (0, 0.0, 0.0);

    for (x1, y1) in asteroids.iter() {
        let mut seen = 0;
        for (x2, y2) in asteroids.iter() {
            if x1 == x2 && y1 == y2 {
                continue;
            }

            let (ax, ay) = (x2 - x1, y2 - y1);

            let mut intersected = false;

            for (x3, y3) in asteroids.iter() {
                if x3 == x1 && y3 == y1 {
                    continue;
                }
                if x3 == x2 && y3 == y2 {
                    continue;
                }

                let (bx, by) = (x3 - x1, y3 - y1);

                if ax * by - ay * bx != 0.0 {
                    continue;
                }

                let alen = ax * ax + ay * ay;
                let blen = bx * bx + by * by;

                if ax * bx + ay * by < 0.0 {
                    continue;
                }

                if alen > blen {
                    intersected = true;
                    break;
                }
            }

            if !intersected {
                seen += 1
            }
        }

        if seen > max.0 {
            max = (seen, *x1, *y1);
        }
    }

    Ok(max)
}

fn main() -> std::io::Result<()> {
    //println!("test5: {:?}", solve("inputs/day10_test5.txt")?);
    println!("test0: {:?}", solve("inputs/day10_test0.txt")?);
    println!("test1: {:?}", solve("inputs/day10_test1.txt")?);
    println!("test2: {:?}", solve("inputs/day10_test2.txt")?);
    println!("test3: {:?}", solve("inputs/day10_test3.txt")?);
    println!("test4: {:?}", solve("inputs/day10_test4.txt")?);
    println!("final: {:?}", solve("inputs/day10.txt")?);
    Ok(())
}
