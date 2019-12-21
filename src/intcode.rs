use crossbeam::crossbeam_channel::bounded;
use std::fs::File;
use std::io::Read;
use std::thread;

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

pub fn run_program<F1, F2>(mut program: Vec<i64>, mut input: F1, output: F2)
where
    F1: FnMut() -> i64,
    F2: Fn(i64),
{
    let mut pc = 0;
    let mut rb = 0;

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

                program[a as usize] = input();
                pc += 2;
            }
            4 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                output(a);
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

            99 => return,

            _ => panic!("unknown opcode {} {}", pc, opcode),
        };
    }
}

pub fn start_intcode_thread(
    program: &Vec<i64>,
) -> (crossbeam::Sender<i64>, crossbeam::Receiver<i64>) {
    let p = program.into_iter().cloned().collect();

    let (s, r) = bounded(0);
    let (s2, r2) = bounded(0);

    let a = move || r.recv().unwrap();
    let b = move |i| s2.send(i).unwrap();

    thread::spawn(move || run_program(p, a, b));

    (s, r2)
}

pub fn load_program(file: &str) -> std::io::Result<Vec<i64>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>())
}
