fn arg(program: &Vec<i64>, pc: usize, mode: i64) -> i64 {
    match mode {
        0 => program[program[pc] as usize],
        1 => program[pc],
        _ => panic!("unknown mode {}", mode),
    }
}

fn run_program(mut program: Vec<i64>, input: &Vec<i64>) {
    let mut pc = 0;
    let mut ic = 0;

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
                println!("{}", a);
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

            99 => return,

            _ => panic!("unknown opcode {} {}", pc, opcode),
        };
    }
}

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day5.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    run_program(program, &vec![1]);
    Ok(())
}
