use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    println!("{}", solve("inputs/day20_0.txt")?);
    println!("{}", solve("inputs/day20_2.txt")?);
     println!("{}", solve("inputs/day20.txt")?);

    Ok(())
}

fn solve(fname: &str) -> std::io::Result<usize> {
    let (maze, start, end) = load_file(fname)?;

    let dist = dijkstra(&maze, start, end);

    Ok(dist[&(0, end)])
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

                    let (portal, portal_from, portal_to, inward) = p.unwrap();
                    println!("{} {:?} {:?} {}", portal, portal_from, portal_to, inward);
                    portals
                        .entry(portal.clone())
                        .or_insert_with(|| Vec::new())
                        .push((portal_from, portal_to, inward));
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

        maze.insert(pps[0].0, Tile::Portal(pps[1].1, pps[0].2));
        maze.insert(pps[1].0, Tile::Portal(pps[0].1, pps[1].2));
    }

    Ok((maze, start, end))
}

#[derive(PartialEq, Eq, Debug)]
struct QueueItem((i64, i64), usize, usize);

fn dijkstra(
    maze: &HashMap<(i64, i64), Tile>,
    start: (i64, i64),
    end: (i64, i64),
) -> HashMap<(usize, (i64, i64)), usize> {
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();

    let mut min = 100000;

    dist.insert((0, start), 0);

    queue.push(QueueItem(start, 0, 0));

    while !queue.is_empty() {
        let q = queue.pop().unwrap();

        println!("{:?}", q);

        if q.1 > min {
            continue;
        }

        if q.0 == end && q.2 == 0 {
            min = q.1;
        }

        let (cx, cy) = q.0;
        let alt = q.1 + 1;

        for (x, y) in vec![(cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)] {
            match *maze.get(&(x, y)).unwrap_or(&Tile::Space) {
                Tile::Space => continue,
                Tile::Wall => continue,

                Tile::Portal(c, inwards) => {
                    if (inwards || q.2 > 0) {
                        let nl = if inwards { q.2 + 1 } else { q.2 - 1 };
                        if alt < *dist.get(&(nl, c)).unwrap_or(&100000) {
                            dist.insert((nl, c.clone()), alt);
                            queue.push(QueueItem(c, alt, nl));
                        }
                    }
                }
                Tile::Passage => {
                    let c = (x, y);
                    if alt < *dist.get(&(q.2, c)).unwrap_or(&100000) {
                        dist.insert((q.2, c.clone()), alt);
                        queue.push(QueueItem(c, alt, q.2));
                    }
                }
            }
        }
    }

    dist
}

fn find_portal(
    maze: &Vec<Vec<char>>,
    pos: (i64, i64),
) -> Option<(String, (i64, i64), (i64, i64), bool)> {
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
                    return Some((
                        vec![a, c].iter().collect(),
                        pos,
                        (px, py),
                        px != 2
                            && py != 2
                            && px != (maze[2].len() - 3) as i64
                            && py != (maze.len() - 3) as i64,
                    ));
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
    Portal((i64, i64), bool),
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
