use crossbeam::crossbeam_channel::bounded;
use crossbeam::Receiver;
use crossbeam::Sender;
use std::thread;

fn arg(program: &Vec<i64>, pc: usize, mode: i64) -> i64 {
    match mode {
        0 => program[program[pc] as usize],
        1 => program[pc],
        _ => panic!("unknown mode {}", mode),
    }
}

fn run_program(mut program: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
    let mut pc = 0;

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

                program[a as usize] = input.recv().unwrap();
                pc += 2;
            }
            4 => {
                let a = arg(&program, pc + 1, a_mode);
                output.send(a).unwrap();
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

fn run_amplifiers(program: &Vec<i64>, inputs: Vec<i64>) -> i64 {
    let (mut s, mut r) = bounded(0);

    let first_send = s.clone();
    let mut last_value = 0;

    for i in inputs {
        let program_copy = program.into_iter().cloned().collect();
        let (s2, r2) = bounded(0);

        let thread_s2 = s2.clone();
        thread::spawn(move || {
            run_program(program_copy, r, thread_s2);
        });
        s.send(i).unwrap();

        s = s2;
        r = r2;
    }

    first_send.send(0).unwrap();

    loop {
        let o = r.recv();

        if o.is_err() {
            break;
        }

        let v = o.unwrap();
        last_value = v;
        match first_send.send(v) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }

    last_value
}

fn max_signal(program: &Vec<i64>) -> (i64, Vec<i64>) {
    let mut max = 0;
    let mut max_v = vec![];
    for a in 5..=9 {
        for b in 5..=9 {
            if a == b {
                continue;
            }

            for c in 5..=9 {
                if a == c || b == c {
                    continue;
                }

                for d in 5..=9 {
                    if a == d || b == d || c == d {
                        continue;
                    }

                    for e in 5..=9 {
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
    let test_program1 = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    println!("{}", run_amplifiers(&test_program1, vec![9, 8, 7, 6, 5]));
    println!("{:?}", max_signal(&test_program1));

    let test_program2 = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    println!("{}", run_amplifiers(&test_program2, vec![9, 7, 8, 5, 6]));
    println!("{:?}", max_signal(&test_program2));

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
