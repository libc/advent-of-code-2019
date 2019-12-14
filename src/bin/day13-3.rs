use aoc2019;
use std::cmp::Ordering;

use crossbeam::crossbeam_channel::bounded;
use crossbeam::crossbeam_channel::unbounded;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time;
extern crate termion;
use crossbeam::select;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as A;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn try_game(p: &Vec<i64>, input: &Vec<i64>, s: crossbeam::Sender<i64>) -> std::io::Result<usize> {
    let mut program: Vec<i64> = p.iter().cloned().collect();
    program[0] = 2;

    let idx = Arc::new(AtomicUsize::new(0));
    let idx2 = idx.clone();

    let s2 = s.clone();

    let a = move || {
        thread::sleep(time::Duration::from_millis(100));
        s2.send(-2).unwrap();
        s2.send(0).unwrap();
        let i = idx2.load(A::Relaxed);
        s2.send(i as i64).unwrap();
        idx2.store(i + 1, A::SeqCst);
        if i >= input.len() {
            0
        } else {
            input[i]
        }
    };

    let b = move |i| s.send(i).unwrap();

    // let mut stdout = stdout().into_raw_mode().unwrap();
    // write!(
    //     stdout,
    //     "{}Score: {:?}                ",
    //     termion::cursor::Goto(1, 1),
    //     input
    // );
    // stdout.flush().unwrap();

    aoc2019::intcode::run_program(program, a, b);

    let i = idx.load(A::Relaxed);

    Ok(i)
}

fn drawer(
    r: crossbeam::Receiver<i64>,
    scores: crossbeam::Sender<i64>,
    grids: crossbeam::Sender<(i64, [[char; 80]; 50])>,
) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut max_score = 0;
    let mut grid = [[' '; 80]; 50];

    loop {
        let xo = r.recv();
        if xo.is_err() {
            return;
        }

        let x = xo.unwrap();
        let y = r.recv().unwrap();
        let tile_id = r.recv().unwrap();

        if x == -1 && y == 0 {
            write!(
                stdout,
                "{}Score: {:10} {:10}                ",
                termion::cursor::Goto(1, 2),
                tile_id,
                max_score
            );

            scores.send(tile_id).unwrap();

            if tile_id > max_score {
                max_score = tile_id;
            }
            continue;
        }
        if x == -2 && y == 0 {
            grids.send((tile_id, grid));
            continue;
        }
        grid[y as usize][x as usize] = match tile_id {
            0 => ' ',
            1 => '#',
            2 => '*',
            3 => '=',
            4 => 'o',
            _ => panic!("uknown tile id {}", tile_id),
        };

        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        );
        for y in 0..30 {
            write!(stdout, "{}", termion::cursor::Goto(1, (y as u16) + 1));
            for x in 0..40 {
                write!(stdout, "{}", grid[y][x]);
            }
        }
        write!(stdout, "{}", termion::cursor::Goto(1, 23));
        for x in 0..40 {
            write!(stdout, "{}", x % 10);
        }
        write!(
            stdout,
            "{}score: {}",
            termion::cursor::Goto(1, 24),
            max_score
        );
        stdout.flush().unwrap();
    }
}

#[derive(PartialEq, Eq)]
struct QueueItem(i64, usize, Vec<i64>);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> std::io::Result<()> {
    // let mut stdout = stdout().into_raw_mode().unwrap();

    // write!(
    //     stdout,
    //     "{}{}{}",
    //     termion::clear::All,
    //     termion::cursor::Goto(1, 1),
    //     termion::cursor::Hide
    // )
    // .unwrap();
    // stdout.flush().unwrap();
    //
    let mut file = File::open("inputs/day13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = contents
        .trim()
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut positions: Vec<(i64, i64)> = vec![
        (788, 25),
        (824, 13),
        (842, 13),
        (878, 25),
        (932, 13),
        (954, 11),
        (1082, 13),
        (1118, 25),
        (1172, 13),
        (1232, 25),
        (1254, 29),
        (1260, 23),
        (1370, 15),
        (1418, 23),
        (1476, 15),
        (1498, 9),
        (1548, 3),
        (1590, 1),
        (1668, 3),
        (1716, 9),
        (1942, 29),
        (2000, 9),
        (2054, 13),
        (2090, 25),
        (2114, 23),
        (2150, 15),
        (2228, 5),
        (2426, 27),
        (2462, 11),
        (2536, 27),
        (2572, 9),
        (2608, 29),
        (2668, 9),
        (2704, 27),
        (2740, 11),
        (2816, 27),
        (2852, 9),
        (2888, 29),
        (2984, 9),
        (3020, 27),
        (3056, 11),
        (3092, 25),
        (3128, 13),
        (3164, 33),
        (3210, 3),
        (3296, 33),
        (3308, 21),
        (3344, 17),
        (3380, 3),
        (3416, 33),
        (3464, 3),
        (3500, 35),
        (3554, 3),
        (3590, 33),
        (3626, 5),
        (3712, 33),
        (3748, 3),
        (3784, 35),
        (3840, 3),
        (3876, 33),
        (3912, 5),
        (3948, 31),
        (3984, 7),
        (4020, 29),
        (4056, 9),
        (4092, 27),
        (4128, 11),
        (4164, 25),
        (4200, 13),
        (4276, 25),
        (4312, 11),
        (4348, 27),
        (4384, 9),
        (4420, 29),
        (4456, 7),
        (4492, 31),
        (4528, 5),
        (4564, 33),
        (4600, 3),
        (4636, 35),
        (4672, 1),
        (4708, 35),
        (4744, 3),
        (4780, 33),
        (4816, 5),
        (4852, 31),
        (4888, 7),
        (4924, 29),
        (4960, 9),
        (4996, 27),
        (5032, 11),
        (5068, 25),
        (5104, 13),
        (5140, 23),
        (5176, 15),
        (5248, 23),
        (5284, 13),
        (5320, 25),
        (5356, 11),
        (5392, 27),
        (5428, 9),
        (5464, 29),
        (5500, 7),
        (5536, 31),
        (5572, 5),
        (5608, 33),
        (5644, 3),
        (5680, 35),
        (5716, 1),
        (5752, 35),
        (5788, 3),
        (5824, 33),
        (5860, 5),
        (5896, 31),
        (5932, 7),
        (5968, 29),
        (6004, 9),
        (6040, 27),
        (6076, 11),
        (6112, 25),
        (6148, 13),
        (6184, 23),
        (6220, 15),
    ];
    loop {
        let (s, r) = bounded(0);
        let (ss, rs) = unbounded();
        let (gs, gr) = unbounded();

        thread::spawn(move || drawer(r, ss, gs));

        let mut qq = vec![
            1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
            -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ];

        println!("{:?}", positions);

        for (p, xx) in positions.iter() {
            while *p >= qq.len() as i64 {
                qq.push(0);
            }

            let ex: i64 = qq.iter().take(*p as usize).sum::<i64>() + 19;
            if ex > *xx {
                for o in 0..(ex - xx) {
                    qq[(p - o) as usize] = -1;
                }
            } else if ex < *xx {
                for o in 0..(xx - ex) {
                    qq[(p - o) as usize] = 1;
                }
            }
        }

        let s2 = s.clone();
        let i = try_game(&program, &qq, s2).unwrap();

        let mut max = 0;
        loop {
            let s = rs.try_recv();
            if s.is_ok() {
                let ss = s.unwrap();
                if ss > max {
                    max = ss;
                }
            } else {
                break;
            }
        }
        println!("{}", max);

        let mut grids = Vec::new();
        loop {
            let g = gr.try_recv();
            if g.is_ok() {
                grids.push(g.unwrap());
            } else {
                break;
            }
        }

        if false {
            let mut idx = grids.len() - 1;
            let mut ball_x = 0;
            let mut ball_y = 0;
            let mut thingies = 0;
            while idx > 0 {
                for y in 0..30 {
                    for x in 0..40 {
                        if grids[idx].1[y][x] == 'o' {
                            ball_x = x;
                            ball_y = y;
                        }
                        if grids[idx].1[y][x] == '*' {
                            thingies += 1;
                        }
                    }
                }

                if thingies == 0 {
                    println!("cleared! {}", max);
                    return Ok(());
                }

                if ball_y == 19 {
                    positions.push((grids[idx].0, ball_x as i64));
                    break;
                } else {
                    idx -= 1;
                }
            }
        } else {
            let stdin = stdin();
            let mut idx = grids.len() - 1;
            let mut paddle_x = 0;
            let mut paddle_y = 0;
            let mut ball_x = 0;
            let mut ball_y = 0;
            let mut stdout = stdout().into_raw_mode().unwrap();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => return Ok(()),
                    Key::Left => {
                        if idx > 0 {
                            idx -= 1
                        }
                    }
                    Key::Right => {
                        if idx < grids.len() - 1 {
                            idx += 1
                        }
                    }
                    _ => {}
                }
                for y in 0..30 {
                    write!(stdout, "{}", termion::cursor::Goto(1, (y as u16) + 1));
                    for x in 0..40 {
                        write!(stdout, "{}", grids[idx].1[y][x]);

                        if grids[idx].1[y][x] == '=' {
                            paddle_x = x;
                            paddle_y = y;
                        }
                        if grids[idx].1[y][x] == 'o' {
                            ball_x = x;
                            ball_y = y;
                        }
                    }
                }
                write!(
                    stdout,
                    "{} {} =: {} {} o: {} {}",
                    termion::cursor::Goto(1, 31),
                    grids[idx].0,
                    paddle_x,
                    paddle_y,
                    ball_x,
                    ball_y
                )
                .unwrap();
                stdout.flush().unwrap();
            }
        }
    }

    Ok(())
}
