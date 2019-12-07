fn run_program(a: usize, b: usize) -> usize {
    let mut program = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 9, 23, 27, 2,
        27, 6, 31, 1, 5, 31, 35, 2, 9, 35, 39, 2, 6, 39, 43, 2, 43, 13, 47, 2, 13, 47, 51, 1, 10,
        51, 55, 1, 9, 55, 59, 1, 6, 59, 63, 2, 63, 9, 67, 1, 67, 6, 71, 1, 71, 13, 75, 1, 6, 75,
        79, 1, 9, 79, 83, 2, 9, 83, 87, 1, 87, 6, 91, 1, 91, 13, 95, 2, 6, 95, 99, 1, 10, 99, 103,
        2, 103, 9, 107, 1, 6, 107, 111, 1, 10, 111, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123,
        13, 127, 1, 127, 5, 131, 1, 6, 131, 135, 2, 135, 13, 139, 1, 139, 2, 143, 1, 143, 10, 0,
        99, 2, 0, 14, 0,
    ];

    program[1] = a;
    program[2] = b;

    let mut pc = 0;

    while program[pc] != 99 {
        match program[pc] {
            1 => {
                let a = program[program[pc + 1]];
                let b = program[program[pc + 2]];
                let c = program[pc + 3];

                program[c] = a + b;
            }
            2 => {
                let a = program[program[pc + 1]];
                let b = program[program[pc + 2]];
                let c = program[pc + 3];

                program[c] = a * b;
            }
            _ => panic!("unknown opcode {}", program[pc]),
        };
        pc += 4;
    }

    program[0]
}

fn main() {
    println!("{}", 19690720);
    let mut a = 12;
    let mut b = 0;

    loop {
        let o = run_program(a, b);
        println!("tried {} {}, got: {}", a, b, o);
        if o < 19690720 {
            a += 1;
        } else {
            a -= 1;
            break;
        }
    }

    loop {
        let o = run_program(a, b);
        println!("tried {} {}, got: {}", a, b, o);

        if o == 19690720 {
            println!("found!");
            return;
        }
        if o < 19690720 {
            b += 1;
        } else {
            b -= 1;
        }
    }
}
