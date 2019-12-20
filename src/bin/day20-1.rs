use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    println!("{}", solve("inputs/day20_0.txt")?);
    println!("{}", solve("inputs/day20_1.txt")?);
    println!("{}", solve("inputs/day20.txt")?);

    Ok(())
}

fn solve(fname: &str) -> std::io::Result<usize> {
    let (maze, start, end) = load_file(fname)?;

    let dist = dijkstra(&maze, start);

    Ok(dist[&end])
}

fn load_file(fname: &str) -> std::io::Result<(HashMap<(i64, i64), Tile>, (i64, i64), (i64, i64))> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut maze = HashMap::new();
    let mut portals = HashMap::new();

    let v = contents
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for (y, l) in v.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            match *t {
                '#' => {
                    maze.insert((x as i64, y as i64), Tile::Wall);
                }
                '.' => {
                    maze.insert((x as i64, y as i64), Tile::Passage);
                }
                ' ' => continue,
                a if a.is_uppercase() => {
                    let p = find_portal(&v, (x as i64, y as i64));
                    if p.is_none() {
                        continue;
                    }

                    let (portal, portal_from, portal_to) = p.unwrap();
                    println!("{} {:?} {:?}", portal, portal_from, portal_to);
                    portals
                        .entry(portal.clone())
                        .or_insert_with(|| Vec::new())
                        .push((portal_from, portal_to));
                }
                _ => panic!("unknown tile {}", t),
            }
        }
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (p, pps) in portals {
        if p == "AA" {
            start = pps[0].1;
            continue;
        }
        if p == "ZZ" {
            end = pps[0].1;
            continue;
        }

        if pps.len() != 2 {
            panic!("portal {} malformed {:?}", p, pps);
        }

        maze.insert(pps[0].0, Tile::Portal(pps[1].1));
        maze.insert(pps[1].0, Tile::Portal(pps[0].1));
    }

    Ok((maze, start, end))
}

#[derive(PartialEq, Eq)]
struct QueueItem((i64, i64), usize);

fn dijkstra(maze: &HashMap<(i64, i64), Tile>, start: (i64, i64)) -> HashMap<(i64, i64), usize> {
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();

    dist.insert(start, 0);

    queue.push(QueueItem(start, 0));

    while !queue.is_empty() {
        let q = queue.pop().unwrap();

        let (cx, cy) = q.0;
        let alt = q.1 + 1;

        for (x, y) in vec![(cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)] {
            match *maze.get(&(x, y)).unwrap_or(&Tile::Space) {
                Tile::Space => continue,
                Tile::Wall => continue,

                Tile::Portal(c) => {
                    if alt < *dist.get(&c).unwrap_or(&100000) {
                        dist.insert(c.clone(), alt);
                        queue.push(QueueItem(c, alt));
                    }
                }
                Tile::Passage => {
                    let c = (x, y);
                    if alt < *dist.get(&c).unwrap_or(&100000) {
                        dist.insert(c.clone(), alt);
                        queue.push(QueueItem(c, alt));
                    }
                }
            }
        }
    }

    dist
}

fn find_portal(maze: &Vec<Vec<char>>, pos: (i64, i64)) -> Option<(String, (i64, i64), (i64, i64))> {
    let (x, y) = pos;
    let a = maze[y as usize][x as usize];

    for (nx, ny) in vec![(x + 1, y), (x, y + 1)] {
        let c = *maze
            .get(ny as usize)
            .and_then(|l| l.get(nx as usize))
            .unwrap_or(&' ');

        if c.is_uppercase() {
            let dx = (x - nx).abs();
            let dy = (y - ny).abs();

            for (pos, (px, py)) in vec![
                ((x, y), (x - dx, y - dy)),
                ((x, y), (x + dx, y + dy)),
                ((nx, ny), (nx + dx, ny + dy)),
                ((nx, ny), (nx - dx, ny - dy)),
            ] {
                if *maze
                    .get(py as usize)
                    .and_then(|l| l.get(px as usize))
                    .unwrap_or(&' ')
                    == '.'
                {
                    return Some((vec![a, c].iter().collect(), pos, (px, py)));
                }
            }
        }
    }

    None
}

enum Tile {
    Wall,
    Space,
    Passage,
    Portal((i64, i64)),
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
