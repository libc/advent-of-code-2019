fn arg(program: &Vec<i64>, pc: usize, rb: i64, mode: i64) -> i64 {
    match mode {
        0 => {
            let a = program[pc] as usize;
            if a >= program.len() {
                0
            } else {
                program[a]
            }
        }
        1 => program[pc],
        2 => {
            let a = (rb + program[pc]) as usize;
            if a >= program.len() {
                0
            } else {
                program[a]
            }
        }
        _ => panic!("unknown mode {}", mode),
    }
}

fn oarg(program: &Vec<i64>, pc: usize, rb: i64, mode: i64) -> usize {
    match mode {
        0 => program[pc] as usize,
        1 => panic!("mad mode {}", mode),
        2 => (rb + program[pc]) as usize,
        _ => panic!("unknown mode {}", mode),
    }
}

fn run_program(mut program: Vec<i64>, input: &Vec<i64>) -> Vec<i64> {
    let mut pc = 0;
    let mut ic = 0;
    let mut rb = 0;
    let mut output = Vec::new();

    loop {
        let opcode = program[pc] % 100;
        let a_mode = (program[pc] / 100) % 10;
        let b_mode = (program[pc] / 1000) % 10;
        let c_mode = (program[pc] / 10000) % 10;

        match opcode {
            1 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);
                let c = oarg(&program, pc + 3, rb, c_mode);

                if c >= program.len() {
                    program.resize(c + 1, 0);
                }

                program[c as usize] = a + b;
                pc += 4;
            }
            2 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);
                let c = oarg(&program, pc + 3, rb, c_mode);

                if c >= program.len() {
                    program.resize(c + 1, 0);
                }

                program[c as usize] = a * b;
                pc += 4;
            }
            3 => {
                let a = oarg(&program, pc + 1, rb, a_mode) as usize;

                if a >= program.len() {
                    program.resize(a + 1, 0);
                }

                program[a as usize] = input[ic];
                ic += 1;
                pc += 2;
            }
            4 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                output.push(a);
                pc += 2;
            }

            5 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);

                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }

            6 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);

                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }

            7 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);
                let c = oarg(&program, pc + 3, rb, c_mode);

                if c >= program.len() {
                    program.resize(c + 1, 0);
                }

                if a < b {
                    program[c as usize] = 1
                } else {
                    program[c as usize] = 0
                }

                pc += 4;
            }

            8 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                let b = arg(&program, pc + 2, rb, b_mode);
                let c = oarg(&program, pc + 3, rb, c_mode);

                if c >= program.len() {
                    program.resize(c + 1, 0);
                }

                if a == b {
                    program[c as usize] = 1
                } else {
                    program[c as usize] = 0
                }

                pc += 4;
            }

            9 => {
                let a = arg(&program, pc + 1, rb, a_mode);

                rb += a;
                pc += 2;
            }

            99 => return output,

            _ => panic!("unknown opcode {} {}", pc, opcode),
        };
    }
}

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!(
        "{:?}",
        run_program(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            &vec![],
        )
    );
    println!(
        "{:?}",
        run_program(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], &vec![],)
    );
    println!(
        "{:?}",
        run_program(vec![104, 1125899906842624, 99], &vec![],)
    );

    let mut file = File::open("inputs/day9.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("program ---");
    println!("{:?}", run_program(program, &vec![1]));

    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("{:?}", run_program(program, &vec![2]));
    Ok(())
}
