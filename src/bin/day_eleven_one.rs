use crossbeam::crossbeam_channel::bounded;
use crossbeam::Receiver;
use crossbeam::Sender;
use std::collections::HashMap;
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

fn run_program(mut program: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
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

                program[a as usize] = input.recv().unwrap();
                pc += 2;
            }
            4 => {
                let a = arg(&program, pc + 1, rb, a_mode);
                output.send(a).unwrap();
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

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day11.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let (s, r) = bounded(0);
    let (s2, r2) = bounded(0);

    let thread_s2 = s2.clone();

    thread::spawn(move || run_program(program, r, thread_s2));

    let mut gg: HashMap<(i64, i64), i64> = HashMap::new();
    let (mut x, mut y) = (500, 500);
    let (mut dx, mut dy) = (0, 1);

    loop {
        let color = gg.get(&(x, y)).unwrap_or(&0);

        if s.send(*color).is_err() {
            break;
        }

        let c = r2.recv();
        if c.is_err() {
            break;
        }
        let a = r2.recv();
        if a.is_err() {
            break;
        }

        match a.unwrap() {
            0 => {
                let (ox, oy) = (dx, dy);
                dx = -oy;
                dy = ox;
            }
            1 => {
                let (ox, oy) = (dx, dy);
                dx = oy;
                dy = -ox;
            }
            _ => panic!("unknown action {}", a.unwrap()),
        }

        gg.insert((x, y), c.unwrap());
        x += dx;
        y += dy;
    }

    println!("{}", gg.len());

    Ok(())
}
