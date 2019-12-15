use aoc2019;
use im::conslist::ConsList;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Move {
    North,
    South,
    East,
    West,
}

impl Move {
    fn perform(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Move::North => (x, y - 1),
            Move::South => (x, y + 1),
            Move::East => (x + 1, y),
            Move::West => (x - 1, y),
        }
    }
}

#[derive(Debug)]
struct QueueItem {
    x: i64,
    y: i64,
    score: i64,
    path: ConsList<Move>,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.score == other.score;
    }
}

impl Eq for QueueItem {}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reachable_tile(grid: &HashMap<(i64, i64), Tile>, x: i64, y: i64, tile: Tile) -> Vec<Move> {
    let mut visited = HashSet::new();

    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

    queue.push(QueueItem {
        x: x,
        y: y,
        score: 0,
        path: ConsList::new(),
    });

    while !queue.is_empty() {
        let item = queue.pop().unwrap();

        let grid_tile = *grid.get(&(item.x, item.y)).unwrap_or(&Tile::Unknown);

        if grid_tile == tile {
            return item.path.reverse().iter().map(|x| (*x).clone()).collect();
        }

        if grid_tile == Tile::Wall {
            continue;
        }

        let mut queue_if_not_visited = |action: Move| {
            let (nx, ny) = action.perform(item.x, item.y);
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push(QueueItem {
                    x: nx,
                    y: ny,
                    score: item.score + 1,
                    path: item.path.cons(action),
                })
            }
        };

        queue_if_not_visited(Move::North);
        queue_if_not_visited(Move::South);
        queue_if_not_visited(Move::West);
        queue_if_not_visited(Move::East);
    }

    vec![]
}

fn pring_grid(grid: &HashMap<(i64, i64), Tile>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in grid.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match *grid.get(&(x, y)).unwrap_or(&Tile::Unknown) {
                Tile::Unknown => print!("?"),
                Tile::Empty => print!(" "),
                Tile::Wall => print!("#"),
                Tile::Oxygen => print!("O"),
            }
        }
        println!("");
    }
}

fn oxygen_fill(grid: &HashMap<(i64, i64), Tile>) -> i64 {
    let mut ox = 0;
    let mut oy = 0;

    for ((x, y), t) in grid {
        if *t == Tile::Oxygen {
            ox = *x;
            oy = *y;
            break;
        }
    }

    println!("found oxygen: {} {}", ox, oy);

    let mut visited = HashSet::new();

    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

    queue.push(QueueItem {
        x: ox,
        y: oy,
        score: 0,
        path: ConsList::new(),
    });

    let mut max_score = 0;

    while !queue.is_empty() {
        let item = queue.pop().unwrap();

        let grid_tile = *grid.get(&(item.x, item.y)).unwrap_or(&Tile::Unknown);

        if grid_tile == Tile::Wall {
            continue;
        }

        let mut queue_if_not_visited = |action: Move| {
            let (nx, ny) = action.perform(item.x, item.y);
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                if item.score == max_score {
                    max_score += 1
                }

                queue.push(QueueItem {
                    x: nx,
                    y: ny,
                    score: item.score + 1,
                    path: item.path.clone(),
                })
            }
        };

        queue_if_not_visited(Move::North);
        queue_if_not_visited(Move::South);
        queue_if_not_visited(Move::West);
        queue_if_not_visited(Move::East);
    }

    max_score
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day15.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

    let mut x = 0;
    let mut y = 0;
    let mut grid = HashMap::new();

    grid.insert((0, 0), Tile::Empty);

    loop {
        let path = reachable_tile(&grid, x, y, Tile::Unknown);

        if path.len() == 0 {
            break;
        }

        for action in path {
            let (nx, ny) = action.perform(x, y);
            match action {
                Move::North => input.send(1).unwrap(),
                Move::South => input.send(2).unwrap(),
                Move::East => input.send(3).unwrap(),
                Move::West => input.send(4).unwrap(),
            }

            let o = output.recv();
            if o.is_err() {
                panic!("error reading")
            }

            match o.unwrap() {
                0 => {
                    grid.insert((nx, ny), Tile::Wall);
                }
                1 => {
                    grid.insert((nx, ny), Tile::Empty);
                    x = nx;
                    y = ny;
                }
                2 => {
                    grid.insert((nx, ny), Tile::Oxygen);
                    x = nx;
                    y = ny;
                }
                _ => panic!("unkonwn output"),
            }
        }
        pring_grid(&grid);
    }

    println!(
        "answer: {}",
        reachable_tile(&grid, 0, 0, Tile::Oxygen).len()
    );

    println!("task 2: {}", oxygen_fill(&grid));

    Ok(())
}
