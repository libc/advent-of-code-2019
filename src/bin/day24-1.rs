use std::collections::HashSet;

fn main() {
    let state = r#"
        ....#
        #..#.
        #..##
        ..#..
        #....
    "#;

    println!("part1 sample: {}", solve(state));

    let state = r#"
        #..#.
        #.#.#
        ...#.
        ....#
        #.#.#
    "#;

    println!("part1: {}", solve(state));
}

fn solve(input: &str) -> u64 {
    let game = input
        .trim()
        .lines()
        .flat_map(|l| {
            l.trim().chars().map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("aah!"),
            })
        })
        .rev()
        .fold(0, |acc, e| (acc << 1) | e);

    let mut states: HashSet<u64> = HashSet::new();

    states.insert(game);

    let mut s = game;

    loop {
        s = step(s);
        if states.contains(&s) {
            print(s);
            return s;
        }
        states.insert(s);
    }
}

fn step(i: u64) -> u64 {
    (0..25)
        .map(|p| {
            let y = p / 5;
            let x = p % 5;

            //  0
            // 1#2
            //  3
            let adj: u64 = vec![
                if y > 0 { bit(i, x, y - 1) } else { false },
                if x > 0 { bit(i, x - 1, y) } else { false },
                if x < 4 { bit(i, x + 1, y) } else { false },
                if y < 4 { bit(i, x, y + 1) } else { false },
            ]
            .into_iter()
            .map(|b| if b { 1 } else { 0 })
            .sum();

            let b = bit(i, x, y);

            if b && adj != 1 {
                0
            } else if !b && (adj == 1 || adj == 2) {
                1
            } else if b {
                1
            } else {
                0
            }
        })
        .rev()
        .fold(0, |acc, e| (acc << 1) | e)
}

fn bit(i: u64, x: u8, y: u8) -> bool {
    i & (1 << (y * 5 + x)) != 0
}

fn print(g: u64) {
    (0..25).for_each(|b| {
        if b % 5 == 0 {
            println!("");
        }

        match g & (1 << b) != 0 {
            true => print!("#"),
            false => print!("."),
        };
    });
    println!("");
}
