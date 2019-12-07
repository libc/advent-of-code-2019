use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn orbit_num(orbits: &HashMap<String, HashSet<String>>, key: String) -> usize {
    let mut totals = 0;
    let set = orbits.get(&key);
    if set.is_none() {
        return 0;
    }

    for os in set.unwrap() {
        totals += orbit_num(orbits, os.clone()) + 1;
    }
    totals
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
        let set = orbits
            .entry(p[1].to_owned())
            .or_insert_with(|| HashSet::new());
        set.insert(p[0].to_owned());
    }

    let mut total_number = 0;

    for key in orbits.keys() {
        total_number += orbit_num(&orbits, key.clone());
    }

    println!("{}", total_number);
}
