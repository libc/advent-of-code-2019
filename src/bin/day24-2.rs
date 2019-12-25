use std::collections::HashMap;

fn main() {
    let input = r#"
        ....#
        #..#.
        #.?##
        ..#..
        #....
    "#;

    let mut s = parse(input);
    print(&s);

    for _ in 0..10 {
        s = step(s);
    }
    print(&s);

    let input = r#"
        #..#.
        #.#.#
        ...#.
        ....#
        #.#.#
    "#;

    let mut s = parse(input);
    print(&s);

    for _ in 0..200 {
        s = step(s);
    }
    print(&s);

    println!("bugs: {}", s.len());
}

fn parse(input: &str) -> HashMap<(i64, u8, u8), Tile> {
    let mut result = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                result.insert((0, x as u8, y as u8), Tile::Space);
            }
            '#' => {
                result.insert((0, x as u8, y as u8), Tile::Bug);
            }
            _ => {}
        })
    });

    result
}

fn print(h: &HashMap<(i64, u8, u8), Tile>) {
    let (min, max) = minmax(h);

    for d in min..=max {
        println!("depth {}", d);

        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    print!("?");
                    continue;
                }

                match h.get(&(d, x, y)).unwrap_or(&Tile::Space) {
                    Tile::Space => print!("."),
                    Tile::Bug => print!("#"),
                }
            }
            println!("");
        }
        println!("");
    }
}

fn minmax(h: &HashMap<(i64, u8, u8), Tile>) -> (i64, i64) {
    h.keys().map(|(d, _, _)| d).fold((0, 0), {
        |(min, max), e| {
            (
                if min > *e { *e } else { min },
                if max < *e { *e } else { max },
            )
        }
    })
}

fn step(s: HashMap<(i64, u8, u8), Tile>) -> HashMap<(i64, u8, u8), Tile> {
    let (min, max) = minmax(&s);

    let mut adj: HashMap<(i64, u8, u8), u64> = HashMap::new();

    for d in (min - 1)..=(max + 1) {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }

                let a = adj_tiles(d, x, y)
                    .into_iter()
                    .map(|c| *s.get(&c).unwrap_or(&Tile::Space))
                    .map(|t| if t == Tile::Bug { 1 } else { 0 })
                    .sum();

                adj.insert((d, x, y), a);
            }
        }
    }

    let mut result = HashMap::new();

    for d in (min - 1)..=(max + 1) {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }

                match s.get(&(d, x, y)).unwrap_or(&Tile::Space) {
                    Tile::Bug => {
                        if *adj.get(&(d, x, y)).unwrap_or(&0) == 1 {
                            result.insert((d, x, y), Tile::Bug);
                        }
                    }
                    Tile::Space => {
                        let a = *adj.get(&(d, x, y)).unwrap_or(&0);
                        if a == 1 || a == 2 {
                            result.insert((d, x, y), Tile::Bug);
                        }
                    }
                }
            }
        }
    }

    result
}

fn adj_tiles(d: i64, x: u8, y: u8) -> Vec<(i64, u8, u8)> {
    let mut adj_vec = Vec::new();
    if x > 0 {
        adj_vec.push((d, x - 1, y));
    } else {
        adj_vec.push((d - 1, 1, 2));
    }
    if x < 4 {
        adj_vec.push((d, x + 1, y));
    } else {
        adj_vec.push((d - 1, 3, 2));
    }
    if y > 0 {
        adj_vec.push((d, x, y - 1));
    } else {
        adj_vec.push((d - 1, 2, 1));
    }
    if y < 4 {
        adj_vec.push((d, x, y + 1));
    } else {
        adj_vec.push((d - 1, 2, 3));
    }
    if y == 1 && x == 2 {
        (0..5).for_each(|ox| adj_vec.push((d + 1, ox, 0)));
    }

    if y == 3 && x == 2 {
        (0..5).for_each(|ox| adj_vec.push((d + 1, ox, 4)));
    }
    if y == 2 && x == 1 {
        (0..5).for_each(|oy| adj_vec.push((d + 1, 0, oy)));
    }
    if y == 2 && x == 3 {
        (0..5).for_each(|oy| adj_vec.push((d + 1, 4, oy)));
    }
    adj_vec
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Space,
    Bug,
}
