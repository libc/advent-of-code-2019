use aoc2019;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

    let mut scene = HashMap::new();

    loop {
        let xo = output.recv();

        if xo.is_err() {
            break;
        }

        let x = xo.unwrap();
        let y = output.recv().unwrap();
        let tile_id = output.recv().unwrap();

        scene.insert((x, y), tile_id);
    }

    let mut block = 0;
    for (_, tile_id) in scene {
        if tile_id == 2 {
            block += 1
        }
    }
    println!("blocks: {}", block);

    Ok(())
}
