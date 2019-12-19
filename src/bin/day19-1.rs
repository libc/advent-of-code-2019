use aoc2019;
use std::fs::File;
use std::io::prelude::*;

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

    for y in 0..50 {
        let mut l = String::new();
        for x in 0..50 {
            let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

            input.send(x).unwrap();
            input.send(y).unwrap();

            match output.recv().unwrap() {
                0 => l += ".",
                1 => {
                    l += "#";
                    p += 1;
                }
                _ => l += "?",
            }
        }
        s += &l;
        s += "\n";
    }

    println!("{}", s);

    println!("answer: {}", p);

    Ok(())
}
