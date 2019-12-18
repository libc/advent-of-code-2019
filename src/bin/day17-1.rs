use aoc2019;
use std::fs::File;
use std::io::prelude::*;
use termion::raw::IntoRawMode;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut s = String::new();
    let (_, output) = aoc2019::intcode::start_intcode_thread(&program);

    for x in output {
        s += &format!("{}", std::char::from_u32(x as u32).unwrap());
    }

    let l = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut o = 0;
    for y in 0..l.len() {
        for x in 0..l[y].len() {
            if l[y][x] != '#' {
                print!("{}", l[y][x]);
                continue;
            }

            let prev_line = l.get(y - 1).and_then(|l| l.get(x));
            let next_line = l.get(y + 1).and_then(|l| l.get(x));
            let prev_char = l[y].get(x - 1);
            let next_char = l[y].get(x + 1);

            let mut is = 0;

            if prev_line.is_some() && *prev_line.unwrap() == '#' {
                is += 1
            }
            if next_line.is_some() && *next_line.unwrap() == '#' {
                is += 1
            }
            if prev_char.is_some() && *prev_char.unwrap() == '#' {
                is += 1
            }
            if next_char.is_some() && *next_char.unwrap() == '#' {
                is += 1
            }

            if is > 2 {
                print!("O");
                o += x * y;
            } else {
                print!("#");
            }
        }
        println!("");
    }

    println!("{}", o);

    Ok(())
}
