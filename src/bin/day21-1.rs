use aoc2019;
use std::collections::HashMap;
use std::thread;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    T,
    J,
}

enum Instruction {
    Not(Register, Register),
    And(Register, Register),
    Or(Register, Register),
    Walk,
}

fn main() -> std::io::Result<()> {
    let program = aoc2019::intcode::load_program("inputs/day21.txt")?;

    let (input, output) = aoc2019::intcode::start_intcode_thread(&program);

    let t = thread::spawn(move || {
        for c in output {
            if c > 255 {
                println!("\n\nscore: {}", c);
                continue;
            }
            print!("{}", (c as u8) as char);
        }
    });

    //program:
    // (!A || !B || !C || !D) && D ||
    //
    // =>
    // !(A && B && C && D) && C
    //
    // t = t || a
    // t = t && b
    // t = t && c
    // t = t && d
    // j = !t
    // j = j && d
    //
    use Instruction::*;
    use Register::*;
    let mut prog = vec![
        Or(A, T),
        And(B, T),
        And(C, T),
        And(D, T),
        Not(T, J),
        And(D, J),
        Walk,
    ];

    let test_cases = vec![
        // @
        // #### ?
        (vec![true, true, true, false], false),
        // @
        // ### #
        (vec![true, true, false, true], false),
        // @
        // ## ##
        (vec![true, false, true, true], true),
        // @
        // # ###
        (vec![false, true, true, true], true),
        // @
        // ###  ?
        (vec![true, true, false, false], false),
        // @
        // ##   ?
        (vec![true, false, false, false], false),
        // @
        // #    ?
        (vec![false, false, false, false], true),
        // @
        // #   #
        (vec![false, false, false, true], true),
    ];

    for (w, j) in test_cases {
        if run_program(&prog, &w) != j {
            println!("failed test case:\n@");
            print!("#");
            for t in w.iter() {
                if *t {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("?");
            println!("expected J = {}, got = {}", j, !j);
            println!("");
        }
    }

    for c in encode_program(&prog).bytes() {
        input.send(c as i64).unwrap();
    }

    t.join().unwrap();

    Ok(())
}

fn run_program(program: &Vec<Instruction>, wall: &Vec<bool>) -> bool {
    let mut values = HashMap::new();
    for (r, v) in vec![Register::A, Register::B, Register::C, Register::D]
        .iter()
        .zip(wall)
    {
        values.insert(*r, *v);
    }

    values.insert(Register::T, false);
    values.insert(Register::J, false);

    for i in program {
        match i {
            Instruction::Walk => return values[&Register::J],
            Instruction::Not(a, b) => {
                values.insert(*b, !values[a]);
            }
            Instruction::And(a, b) => {
                values.insert(*b, values[a] && values[b]);
            }
            Instruction::Or(a, b) => {
                values.insert(*b, values[a] || values[b]);
            }
        }
    }

    false
}

fn encode_program(program: &Vec<Instruction>) -> String {
    program
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("\n")
        + &"\n"
}

impl Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Not(a, b) => format!("NOT {} {}", a.to_string(), b.to_string()),
            Instruction::And(a, b) => format!("AND {} {}", a.to_string(), b.to_string()),
            Instruction::Or(a, b) => format!("OR {} {}", a.to_string(), b.to_string()),
            Instruction::Walk => format!("WALK"),
        }
    }
}

impl Register {
    fn to_string(&self) -> String {
        String::from(match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::T => "T",
            Register::J => "J",
        })
    }
}
