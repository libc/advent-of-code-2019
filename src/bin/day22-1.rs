use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    solve(10, "inputs/day22-0.txt")?;
    solve(10, "inputs/day22-1.txt")?;
    solve(10, "inputs/day22-2.txt")?;
    solve(10, "inputs/day22-3.txt")?;
    solve(10007, "inputs/day22.txt")?;
    Ok(())
}

fn solve(n: usize, fname: &str) -> std::io::Result<()>{
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut stack : Vec<i64> = (0..n).map(|x| x as i64).collect();

    for line in contents.lines() {
        if line.starts_with("deal with increment ") {
            stack = deal_with_increment(&stack, line[20..].parse::<i64>().unwrap());
        }

        if line == "deal into new stack" {
            stack = deal(&stack);
        }


        if line.starts_with("cut ") {
            stack = cut(&stack, line[4..].parse::<i64>().unwrap());
        }
    }

    if stack.len() < 20 {
        println!("{} stack: {:?}", fname, stack);
    } else {
        for (p, c) in stack.iter().enumerate() {
            if *c == 2019 {
                println!("{} 2019th card: {}", fname, p);
            }
        }
    }

    Ok(())
}

fn deal(old_stack: &Vec<i64>) -> Vec<i64> {
    old_stack.iter().cloned().rev().collect()
}

fn deal_with_increment(old_stack: &Vec<i64>, incr: i64) -> Vec<i64> {
    let mut new_stack = vec![0; old_stack.len()];
    let mut idx = 0;

    for c in old_stack {
        new_stack[idx] = *c;
        idx += incr as usize;
        if idx > old_stack.len() { idx -= old_stack.len() }
    }

    new_stack
}

fn cut(old_stack: &Vec<i64>, cut: i64) -> Vec<i64> {
    let cut_fixed = if cut < 0 {
        old_stack.len() as i64 + cut
    } else {
        cut
    } as usize;

    old_stack[cut_fixed..].iter().chain(old_stack[0..cut_fixed].iter()).cloned().collect()
}
