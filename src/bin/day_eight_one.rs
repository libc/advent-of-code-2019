use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day8.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut min = 25 * 6;
    let mut answer = 0;
    let mut zeroes = 0;
    let mut ones = 0;
    let mut twos = 0;

    for (i, c) in contents.trim().chars().enumerate() {
        match c {
            '0' => zeroes += 1,
            '1' => ones += 1,
            '2' => twos += 1,
            _ => continue,
        }

        if i > 0 && i % (25 * 6) == 0 {
            if zeroes < min {
                min = zeroes;
                answer = ones * twos;
            }

            zeroes = 0;
            ones = 0;
            twos = 0;
        }
    }

    println!("{}", answer);

    Ok(())
}
