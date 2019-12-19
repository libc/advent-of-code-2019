use aoc2019;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Unknown,
    Space,
    Beam,
}

fn check_tile(program: &Vec<i64>, x: i64, y: i64) -> Tile {
    let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

    input.send(x).unwrap();
    input.send(y).unwrap();

    match output.recv().unwrap() {
        0 => Tile::Space,
        1 => Tile::Beam,
        _ => Tile::Unknown,
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day19.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut s = String::new();
    let mut p = 0;

    let mut grid = HashMap::new();

    for y in 0..10 {
        for x in 0..10 {
            grid.insert((x, y), check_tile(&program, x, y));
        }
    }

    let mut y = 9;
    let mut lx = 0;
    let mut rx;

    loop {
        y += 1;
        loop {
            let t = check_tile(&program, y, lx);
            grid.insert((lx, y), t);
            if t == Tile::Beam {
                break;
            }
            lx += 1;
        }

        rx = lx + 1;
        loop {
            let t = check_tile(&program, y, rx);
            grid.insert((rx, y), t);
            if t != Tile::Beam {
                break;
            }
            rx += 1;
        }

        if rx - lx > 100 {
            let top_row = (lx..(lx + 100))
                .all(|x| *grid.get(&(x, y - 99)).unwrap_or(&Tile::Unknown) == Tile::Beam);
            let left_col =
                ((y - 99)..=y).all(|y| *grid.get(&(lx, y)).unwrap_or(&Tile::Unknown) == Tile::Beam);
            let right_col = ((y - 99)..=y)
                .all(|y| *grid.get(&(lx + 99, y)).unwrap_or(&Tile::Unknown) == Tile::Beam);

            println!(
                "checking {} top: {} left: {} right: {}, lx: {}, rx: {}",
                y, top_row, left_col, right_col, lx, rx
            );

            if top_row && left_col && right_col {
                for yy in y - 110..=y {
                    for xx in lx - 10..rx {
                        match *grid.get(&(xx, yy)).unwrap_or(&Tile::Unknown) {
                            Tile::Unknown => print!(" "),
                            Tile::Space => print!("."),
                            Tile::Beam => print!("#"),
                        }
                    }
                    println!("");
                }

                println!("santa fits at: {} {}", y - 99, lx);
                break;
            }
        }
    }

    println!("{}", s);

    println!("answer: {}", p);

    Ok(())
}
