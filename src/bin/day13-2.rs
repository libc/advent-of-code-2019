use aoc2019;
use std::cmp::Ordering;

use crossbeam::crossbeam_channel::bounded;
use crossbeam::crossbeam_channel::unbounded;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::thread;
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

    let a = move || {
        let i = idx2.load(A::Relaxed);
        idx2.store(i + 1, A::SeqCst);
        if i >= input.len() + 2000 {
            1
        } else if i >= input.len() + 1000 {
            -1
        } else if i >= input.len() {
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

fn drawer(r: crossbeam::Receiver<i64>, scores: crossbeam::Sender<i64>) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut max_score = 0;
    let mut grid = [[' '; 80]; 50];
    let mut i = 0;
    loop {
        let xo = r.recv();
        if xo.is_err() {
            return;
        }

        let x = xo.unwrap();
        let y = r.recv().unwrap();
        let tile_id = r.recv().unwrap();

        if x == -1 && y == 0 {
            // write!(
            //     stdout,
            //     "{}Score: {:10} {:10}                ",
            //     termion::cursor::Goto(1, 2),
            //     tile_id,
            //     max_score
            // );

            scores.send(tile_id).unwrap();

            if tile_id > max_score {
                max_score = tile_id;
            }
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

        if i == 1000 {
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
            write!(
                stdout,
                "{}score: {}",
                termion::cursor::Goto(1, 31),
                max_score
            );
            stdout.flush().unwrap();
            i = 0;
        } else {
            i += 1;
        }
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

    let (s, r) = bounded(0);
    let (ss, rs) = unbounded();

    thread::spawn(move || drawer(r, ss));

    let mut q = BinaryHeap::new();

    q.push(QueueItem(0, 0, vec![0]));
    q.push(QueueItem(0, 0, vec![1]));
    q.push(QueueItem(0, 0, vec![-1]));

    while !q.is_empty() {
        let qi = q.pop().unwrap();
        let qq = qi.2;

        let s2 = s.clone();
        let consumed = try_game(&program, &qq, s2).unwrap();
        if consumed < qq.len() {
            continue;
        }

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

        for i in vec![-1, 1, 0] {
            let mut qqq: Vec<i64> = qq.iter().map(|x| x).cloned().collect();
            qqq.push(i);
            q.push(QueueItem(max, consumed, qqq));
        }
    }

    Ok(())
}
