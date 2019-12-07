use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq)]
struct QueueItem(usize, String);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let fname = env::args()
        .into_iter()
        .nth(1)
        .unwrap_or(String::from("inputs/day6.txt"));

    println!("{}", fname);

    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut orbits = HashMap::new();

    for pairs in contents.trim().split("\n") {
        let p: Vec<&str> = pairs.split(")").collect();
        orbits
            .entry(p[1].to_owned())
            .or_insert_with(|| HashSet::new())
            .insert(p[0].to_owned());
        orbits
            .entry(p[0].to_owned())
            .or_insert_with(|| HashSet::new())
            .insert(p[1].to_owned());
    }

    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::new();

    queue.push(QueueItem(0, "YOU".to_owned()));

    while !queue.is_empty() {
        let item = queue.pop().unwrap();

        let set = orbits.get(&item.1);

        if set.is_none() {
            continue;
        }

        for ss in set.unwrap() {
            if visited.contains(ss) {
                continue;
            }

            if ss == "SAN" {
                println!("found: {}", item.0 - 1);
            }

            visited.insert(ss);
            queue.push(QueueItem(item.0 + 1, ss.clone()));
        }
    }
}
