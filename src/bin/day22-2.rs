use mod_exp::mod_exp;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    println!(
        "part2: {}",
        solve(119315717514047, 2020, 101741582076661, "inputs/day22.txt")?
    );
    Ok(())
}

fn solve(n: i128, p: i128, i: i128, fname: &str) -> std::io::Result<i128> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (mut a, mut b) = (1, 0);

    for line in contents.lines().rev() {
        if line.starts_with("deal with increment ") {
            let mut off = line[20..].parse::<i128>().unwrap();

            off = mod_exp(off, n - 2, n) % n;

            a = a * off % n;
            b = b * off % n;
        }

        if line == "deal into new stack" {
            a = -a;
            b = -b - 1;
        }

        if line.starts_with("cut ") {
            b += line[4..].parse::<i128>().unwrap();
        }
    }

    b = (b * ((mod_exp(a, i, n) - 1) * mod_exp(a - 1, n - 2, n) % n)) % n;
    a = mod_exp(a, i, n) % n;
    Ok(((a * p) % n + b) % n)
}
