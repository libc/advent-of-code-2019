use aoc2019;
use im::HashSet;
use im::Vector;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    program[0] = 2;

    let mut s = String::new();
    let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

    // A: L,12,L,8,R,12,
    // B: L,10,L,8,L,12,R,12,
    // A: L,12,L,8,R,12,
    // C: R,12,L,8,L,10,
    // A: L,12,L,8,R,12,
    // A: L,12,L,8,R,12,
    // C: R,12,L,8,L,10,
    // B: L,10,L,8,L,12,R,12,
    // C: R,12,L,8,L,10,
    // B: L,10,L,8,L,12,R,12

    thread::spawn(move || {
        for c in "A,B,A,C,A,A,C,B,C,B\nL,12,L,8,R,12\nL,10,L,8,L,12,R,12\nR,12,L,8,L,10\nn\n".bytes()
        {
            input.send(c as i64).unwrap();
        }
    });

    for x in output {
        if x < 255 {
            print!("{}", std::char::from_u32(x as u32).unwrap());
        } else {
            println!("\nanswer: {}", x);
        }
    }

    return Ok(());

    let l = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    find_paths(l, |path| {
        println!("{}", optimize(&path));
        let (program, a, b, c) = compress(path);
        println!("{:?} {} {} {}", program, a, b, c);

        program.len() > 0
    });

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
enum Program {
    A,
    B,
    C,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Forward,
}

fn optimize(v: &[Move]) -> String {
    let mut s = String::new();

    let mut moves = 0;
    let mut append = |p: &str| {
        if s.len() > 0 {
            s += ",";
        }
        s += p;
    };

    for m in v {
        match m {
            Move::Left => {
                if moves > 0 {
                    append(&format!("{}", moves));
                    moves = 0;
                }
                append("L");
            }
            Move::Right => {
                if moves > 0 {
                    append(&format!("{}", moves));
                    moves = 0;
                }
                append("R");
            }
            Move::Forward => moves += 1,
        }
    }
    if moves > 0 {
        append(&format!("{}", moves));
    }

    s
}

fn compress(path: Vec<Move>) -> (Vec<Program>, String, String, String) {
    for a_idx in 0..path.len() {
        let a = &path[0..=a_idx];
        if optimize(a).len() > 20 {
            continue;
        }
        for b_idx in (a_idx + 1)..path.len() {
            let b = &path[(a_idx + 1)..=b_idx];
            if optimize(b).len() > 20 {
                continue;
            }

            for c_idx in (b_idx + 1)..path.len() {
                let c = &path[(b_idx + 1)..=c_idx];
                if optimize(c).len() > 20 {
                    continue;
                }

                let mut idx = 0;
                let mut program = Vec::new();
                while idx < path.len() {
                    if idx + a.len() <= path.len() && path[idx..(idx + a.len())] == *a {
                        idx += a.len();
                        program.push(Program::A);
                        continue;
                    }
                    if idx + b.len() <= path.len() && path[idx..(idx + b.len())] == *b {
                        idx += b.len();
                        program.push(Program::B);
                        continue;
                    }
                    if idx + c.len() <= path.len() && path[idx..(idx + c.len())] == *c {
                        idx += c.len();
                        program.push(Program::C);
                        continue;
                    }
                    break;
                }

                if idx == path.len() {
                    return (program, optimize(a), optimize(b), optimize(c));
                }
            }
        }
    }

    (vec![], String::new(), String::new(), String::new())
}

fn stepable(x: char) -> bool {
    x == '#' || x == '>' || x == '<' || x == '^' || x == 'v'
}

#[derive(Debug, Eq, PartialEq)]
struct QueueItem {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    path: Vector<Move>,
    points_to_visit: HashSet<(i64, i64)>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .points_to_visit
            .len()
            .cmp(&self.points_to_visit.len())
            .then(other.path.len().cmp(&self.path.len()))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_paths<F>(grid: Vec<Vec<char>>, f: F)
where
    F: Fn(Vec<Move>) -> bool,
{
    let mut points_to_visit = HashSet::new();
    let mut robot_y = 0;
    let mut robot_x = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                points_to_visit = points_to_visit.insert((x as i64, y as i64));
            }

            if grid[y][x] == '>' || grid[y][x] == '^' || grid[y][x] == 'v' || grid[y][x] == '<' {
                robot_y = y;
                robot_x = x;
            }
        }
    }

    let mut queue = BinaryHeap::new();

    match grid[robot_y][robot_x] {
        '^' => {
            queue.push(QueueItem {
                x: robot_x as i64,
                y: robot_y as i64,
                dx: 0,
                dy: -1,
                path: Vector::new(),
                points_to_visit: points_to_visit,
            });
        }
        _ => panic!("unsupported"),
    }

    while queue.is_empty() == false {
        let p = queue.pop().unwrap();

        if p.points_to_visit.is_empty() {
            if f(p.path.iter().map(|x| *x.clone()).collect()) {
                return;
            } else {
                continue;
            }
        }

        {
            let nx = p.x + p.dx;
            let ny = p.y + p.dy;

            let n = grid.get(ny as usize).and_then(|l| l.get(nx as usize));

            if n.is_some() && stepable(*n.unwrap()) {
                queue.push(QueueItem {
                    x: nx,
                    y: ny,
                    dx: p.dx,
                    dy: p.dy,
                    path: p.path.push_back(Move::Forward),
                    points_to_visit: p.points_to_visit.remove(&(nx, ny)),
                });
            }
        }

        {
            let ndx = -p.dy;
            let ndy = p.dx;
            let nx = p.x + ndx;
            let ny = p.y + ndy;

            let n = grid.get(ny as usize).and_then(|l| l.get(nx as usize));

            if n.is_some() && stepable(*n.unwrap()) {
                queue.push(QueueItem {
                    x: nx,
                    y: ny,
                    dx: ndx,
                    dy: ndy,
                    path: p.path.push_back(Move::Right).push_back(Move::Forward),
                    points_to_visit: p.points_to_visit.remove(&(nx, ny)),
                });
            }
        }

        {
            let ndx = p.dy;
            let ndy = -p.dx;
            let nx = p.x + ndx;
            let ny = p.y + ndy;

            let n = grid.get(ny as usize).and_then(|l| l.get(nx as usize));

            if n.is_some() && stepable(*n.unwrap()) {
                queue.push(QueueItem {
                    x: nx,
                    y: ny,
                    dx: ndx,
                    dy: ndy,
                    path: p.path.push_back(Move::Left).push_back(Move::Forward),
                    points_to_visit: p.points_to_visit.remove(&(nx, ny)),
                });
            }
        }
    }
}
