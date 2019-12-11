use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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

fn best_position(fname: &str) -> std::io::Result<(usize, f64, f64)> {
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

struct QueueItem(u64, f64, f64);

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1 && self.2 == other.2;
    }
}

impl Eq for QueueItem {}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn evaporation(fname: &str, x: f64, y: f64) -> std::io::Result<Vec<(f64, f64)>> {
    let asteroids = load_file(fname)?;
    let mut order = Vec::new();
    let mut points = HashMap::new();

    let (ax, ay) = (x - x, -1.0 - y);

    for (x1, y1) in asteroids {
        if x1 == x && y1 == y {
            continue;
        }

        let (bx, by) = (x1 - x, y1 - y);

        let dot = ax * bx + ay * by;
        let det = ax * by - ay * bx;

        let mut angle = det.atan2(dot);

        if angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }

        points
            .entry((angle * 10000000.0).round() as i64)
            .or_insert_with(|| BinaryHeap::new())
            .push(QueueItem(
                ((bx * bx + by * by).sqrt() * 100000.0).round() as u64,
                x1,
                y1,
            ));
    }

    let mut keys = points.keys().into_iter().cloned().collect::<Vec<i64>>();
    keys.sort();

    loop {
        let mut pushed = false;
        for k in keys.iter() {
            let heap = points.get_mut(&k).unwrap();
            if heap.is_empty() {
                continue;
            }

            let i = heap.pop().unwrap();

            order.push((i.1, i.2));
            pushed = true;
        }

        if !pushed {
            break;
        }
    }

    println!("{:?}", order);

    Ok(order)
}

fn solve(fname: &str) -> std::io::Result<(f64, f64)> {
    let (_, x, y) = best_position(fname)?;

    let order = evaporation(fname, x, y)?;

    use std::io::{Error, ErrorKind};
    let custom_error = Error::new(ErrorKind::Other, "oh no!");

    for (i, (x, y)) in order.iter().enumerate() {
        println!("{} {} {}", i, x, y);
    }

    order.get(199).map(|&a| a).ok_or(custom_error)
}

fn main() -> std::io::Result<()> {
    //println!("test5: {:?}", solve("inputs/day10_test5.txt")?);
    // println!("test0: {:?}", solve("inputs/day10_test0.txt")?);
    // println!("test1: {:?}", solve("inputs/day10_test1.txt")?);
    // println!("test2: {:?}", solve("inputs/day10_test2.txt")?);
    // println!("test3: {:?}", solve("inputs/day10_test3.txt")?);
    println!("test4: {:?}", solve("inputs/day10_test4.txt")?);
    println!("final: {:?}", solve("inputs/day10.txt")?);
    // println!("test6: {:?}", solve("inputs/day10_test6.txt")?);
    Ok(())
}
