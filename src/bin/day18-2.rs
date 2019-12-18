use im::HashSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    println!("{}", solve("inputs/day18_2-0.txt")?);
    println!("{}", solve("inputs/day18_2-1.txt")?);
    println!("{}", solve("inputs/day18_2-2.txt")?);
    println!("{}", solve("inputs/day18_2-3.txt")?);
    println!("{}", solve("inputs/day18_2.txt")?);
    Ok(())
}

fn node_id(collected_keys: &HashSet<char>, node: char) -> String {
    let mut keys = collected_keys
        .iter()
        .map(|x| *x.clone())
        .collect::<Vec<char>>();

    keys.sort();

    format!("<{}>{}", keys.iter().collect::<String>(), node)
}

fn solve(file: &str) -> std::io::Result<usize> {
    let puzzle = load_file(file)?;

    let all_keys = find_all_keys(&puzzle);

    let mut queue = BinaryHeap::new();
    let mut min_steps = 100000;

    queue.push(QueueItem {
        nid: String::from("<>@"),
        pos: find_start_pos(&puzzle),
        collected_keys: HashSet::new(),
    });

    let mut connections = HashMap::new();
    let mut queued = HashSet::new();

    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        // println!("{}", item.nid);

        connections
            .entry(item.nid.clone())
            .or_insert_with(|| HashMap::new());

        for (pos_i, ipos) in item.pos.iter().enumerate() {
            let accessible_keys = find_reachable_keys(&puzzle, &item.collected_keys, *ipos);

            for (key, steps, pos) in accessible_keys {
                let nnid = node_id(&item.collected_keys, key);

                let old_value = connections
                    .get_mut(&item.nid)
                    .unwrap()
                    .entry(nnid.clone())
                    .or_insert(steps);
                if *old_value > steps {
                    *old_value = steps
                }

                if !queued.contains(&nnid) {
                    let new_keys = item.collected_keys.insert(key);
                    queued = queued.insert(nnid.clone());
                    let mut npos = item.pos.iter().cloned().collect::<Vec<(i64, i64)>>();
                    npos[pos_i] = pos;
                    queue.push(QueueItem {
                        nid: nnid.clone(),
                        collected_keys: new_keys,
                        pos: npos,
                    });
                }
            }
        }
    }

    let mut dist = HashMap::new();

    dist.insert(String::from("<>@"), 0);

    let mut queue = BinaryHeap::new();

    queue.push(DijkstraQueue {
        node: String::from("<>@"),
        dist: 0,
    });

    while !queue.is_empty() {
        let item = queue.pop().unwrap();

        for (c, d) in connections.get(&item.node).unwrap() {
            let alt = dist.get(&item.node).unwrap_or(&100000) + d;

            if alt < *dist.get(c).unwrap_or(&100000) {
                dist.insert(c.clone(), alt);
                queue.push(DijkstraQueue {
                    node: c.clone(),
                    dist: alt,
                });
            }
        }
    }

    println!("{:?}", dist);

    let mut max_key_len = 0;

    for (k, _) in dist.iter() {
        if k.len() > max_key_len {
            max_key_len = k.len()
        }
    }

    for (k, v) in dist.iter() {
        if k.len() == max_key_len && *v < min_steps {
            min_steps = *v
        }
    }

    Ok(min_steps)
}

#[derive(PartialEq, Eq, Debug)]
struct QueueItem {
    nid: String,
    pos: Vec<(i64, i64)>,
    collected_keys: HashSet<char>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.collected_keys.len().cmp(&other.collected_keys.len())
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct DijkstraQueue {
    node: String,
    dist: usize,
}

impl Ord for DijkstraQueue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for DijkstraQueue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn load_file(fname: &str) -> std::io::Result<Vec<Vec<Tile>>> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut puzzle = Vec::new();
    for l in contents.lines() {
        let mut line = Vec::new();
        for c in l.chars() {
            match c {
                '#' => line.push(Tile::Wall),
                '@' => line.push(Tile::StartPosition),
                '.' => line.push(Tile::Space),
                d if d.is_lowercase() => line.push(Tile::Key(d)),
                d if d.is_uppercase() => line.push(Tile::Door(d.to_lowercase().nth(0).unwrap())),
                _ => panic!("unknown tile {}", c),
            }
        }
        puzzle.push(line)
    }

    Ok(puzzle)
}

fn find_start_pos(puzzle: &Vec<Vec<Tile>>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for (y, l) in puzzle.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            match t {
                Tile::StartPosition => result.push((x as i64, y as i64)),
                _ => continue,
            }
        }
    }

    result
}

fn find_all_keys(puzzle: &Vec<Vec<Tile>>) -> HashSet<char> {
    let mut result = HashSet::new();

    for l in puzzle.iter() {
        for t in l.iter() {
            match t {
                Tile::Key(k) => result = result.insert(k),
                _ => continue,
            }
        }
    }

    result
}

fn find_reachable_keys(
    puzzle: &Vec<Vec<Tile>>,
    collected_keys: &HashSet<char>,
    pos: (i64, i64),
) -> Vec<(char, usize, (i64, i64))> {
    let mut queue: BinaryHeap<KeyFinderQueueItem> = BinaryHeap::new();
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    queue.push(KeyFinderQueueItem {
        x: pos.0,
        y: pos.1,
        score: 0,
    });

    while !queue.is_empty() {
        let item = queue.pop().unwrap();

        let t = puzzle
            .get(item.y as usize)
            .and_then(|l| l.get(item.x as usize))
            .unwrap_or(&Tile::Wall);

        match *t {
            Tile::Wall => continue,
            Tile::Door(d) => {
                if !collected_keys.contains(&d) {
                    continue;
                }
            }
            Tile::Key(d) => {
                if !collected_keys.contains(&d) {
                    result.push((d, item.score, (item.x, item.y)));
                    continue;
                }
            }
            Tile::Space => {}
            Tile::StartPosition => {}
        };

        let mut queue_if_not_visited = |dx: i64, dy: i64| {
            let (nx, ny) = (item.x + dx, item.y + dy);

            if !visited.contains(&(nx, ny)) {
                visited = visited.insert((nx, ny));
                queue.push(KeyFinderQueueItem {
                    x: nx,
                    y: ny,
                    score: item.score + 1,
                })
            }
        };

        queue_if_not_visited(0, 1);
        queue_if_not_visited(0, -1);
        queue_if_not_visited(1, 0);
        queue_if_not_visited(-1, 0);
    }

    result
}

enum Tile {
    Space,
    Wall,
    Key(char),
    Door(char),
    StartPosition,
}

#[derive(PartialEq, Eq)]
struct KeyFinderQueueItem {
    x: i64,
    y: i64,
    score: usize,
}

impl Ord for KeyFinderQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for KeyFinderQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
