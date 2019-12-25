use aoc2019;

fn main() {
    let p = aoc2019::intcode::load_program("inputs/day25.txt").unwrap();

    let mut line = String::new();
    let input = || {
        while line.len() == 0 {
            std::io::stdin().read_line(&mut line).unwrap();
        }

        line.remove(0) as i64
    };

    let output = |i| {
        print!("{}", (i as u8) as char);
    };

    aoc2019::intcode::run_program(p, input, output);
}
