fn arg(program: &Vec<i64>, pc: usize, mode: i64) -> i64 {
    match mode {
        0 => program[program[pc] as usize],
        1 => program[pc],
        _ => panic!("unknown mode {}", mode),
    }
}

fn run_program(mut program: Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
    let mut pc = 0;
    let mut ic = 0;
    let mut output = Vec::new();

    loop {
        let opcode = program[pc] % 100;
        let a_mode = (program[pc] / 100) % 10;
        let b_mode = (program[pc] / 1000) % 10;
        // let c_mode = (program[pc] / 10000) % 10;

        match opcode {
            1 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);
                let c = arg(&program, pc + 3, 1);

                program[c as usize] = a + b;
                pc += 4;
            }
            2 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);
                let c = arg(&program, pc + 3, 1);

                program[c as usize] = a * b;
                pc += 4;
            }
            3 => {
                let a = arg(&program, pc + 1, 1);

                program[a as usize] = input[ic];
                ic += 1;
                pc += 2;
            }
            4 => {
                let a = arg(&program, pc + 1, a_mode);
                output.push(a);
                pc += 2;
            }

            5 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);

                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }

            6 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);

                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }

            7 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);
                let c = arg(&program, pc + 3, 1);

                if a < b {
                    program[c as usize] = 1
                } else {
                    program[c as usize] = 0
                }

                pc += 4;
            }

            8 => {
                let a = arg(&program, pc + 1, a_mode);
                let b = arg(&program, pc + 2, b_mode);
                let c = arg(&program, pc + 3, 1);

                if a == b {
                    program[c as usize] = 1
                } else {
                    program[c as usize] = 0
                }

                pc += 4;
            }

            99 => return output,

            _ => panic!("unknown opcode {} {}", pc, opcode),
        };
    }
}

fn run_amplifiers(program: &Vec<i64>, inputs: Vec<i64>) -> i64 {
    let mut previous_output = 0;
    for i in inputs {
        let next_output = run_program(
            program.into_iter().cloned().collect(),
            &vec![i, previous_output],
        );

        previous_output = next_output[0];
    }

    previous_output
}

fn max_signal(program: &Vec<i64>) -> (i64, Vec<i64>) {
    let mut max = 0;
    let mut max_v = vec![];
    for a in 0..=4 {
        for b in 0..=4 {
            if a == b {
                continue;
            }

            for c in 0..=4 {
                if a == c || b == c {
                    continue;
                }

                for d in 0..=4 {
                    if a == d || b == d || c == d {
                        continue;
                    }

                    for e in 0..=4 {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }

                        let o = run_amplifiers(program, vec![a, b, c, d, e]);
                        if o > max {
                            max = o;
                            max_v = vec![a, b, c, d, e];
                        }
                    }
                }
            }
        }
    }

    (max, max_v)
}

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!(
        "{}",
        run_amplifiers(
            &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
            vec![4, 3, 2, 1, 0]
        )
    );
    println!(
        "{:?}",
        max_signal(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ])
    );

    println!(
        "{}",
        run_amplifiers(
            &vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],
            vec![0, 1, 2, 3, 4]
        )
    );
    println!(
        "{:?}",
        max_signal(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])
    );

    println!(
        "{}",
        run_amplifiers(
            &vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],
            vec![1, 0, 4, 3, 2]
        )
    );
    println!(
        "{:?}",
        max_signal(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ])
    );

    let mut file = File::open("inputs/day7.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("{:?}", max_signal(&program));

    Ok(())
}
