use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day8.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut image = vec![2; 25 * 6];

    for (i, c) in contents.trim().chars().enumerate() {
        let p = i % (25 * 6);

        if image[p] != 2 {
            continue;
        }

        match c {
            '0' => image[p] = 0,
            '1' => image[p] = 1,
            '2' => continue,
            _ => continue,
        }
    }

    for (i, c) in image.iter().enumerate() {
        if i % 25 == 0 {
            println!("");
        }

        match c {
            0 => print!(" "),
            1 => print!("\x1b[7m \x1b[m"),
            2 => print!("-"),
            _ => panic!("wtf?"),
        }
    }
    println!("");

    Ok(())
}
