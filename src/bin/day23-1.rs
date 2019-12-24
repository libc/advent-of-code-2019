use aoc2019;
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::crossbeam_channel::Select;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn nic_thread(
    program: &Vec<i64>,
    n: i64,
) -> (
    crossbeam::Sender<(i64, i64)>,
    crossbeam::Receiver<(i64, i64, i64)>,
) {
    let (s, r) = unbounded();
    let (s2, r2) = unbounded();

    let mut other = Some(n);
    let mut v = Vec::new();

    let p = program.into_iter().cloned().collect();

    thread::Builder::new()
        .name(format!("nic-{}", n))
        .spawn(move || {
            let a = || {
                if other.is_some() {
                    let v = other.unwrap();
                    other = None;
                    v
                } else {
                    match r2.try_recv() {
                        Ok((x, y)) => {
                            other = Some(y);
                            x
                        }
                        _ => -1,
                    }
                }
            };

            let b = |i| {
                v.push(i);
                if v.len() == 3 {
                    s.send((v[0], v[1], v[2])).unwrap();
                    v = Vec::new();
                }
            };
            aoc2019::intcode::run_program(p, a, b);
        })
        .unwrap();

    (s2, r)
}

fn main() -> std::io::Result<()> {
    let program = aoc2019::intcode::load_program("inputs/day23.txt")?;

    let mut ss = Vec::new();
    let mut rs = Vec::new();
    let mut sel = Select::new();
    let mut hash = HashMap::new();

    for i in 0..50 {
        let (s, r) = nic_thread(&program, i);

        ss.push(s);
        rs.push(r);
    }

    for i in 0..50 {
        hash.insert(sel.recv(&rs[i]), i);
    }

    let mut last_xy = (0, 0);
    let mut first_255 = true;

    loop {
        let oper_r = sel.select_timeout(Duration::from_millis(500));
        if oper_r.is_err() {
            println!("sending: {:?}", last_xy);
            ss[0].send(last_xy).unwrap();
            continue;
        }

        let oper = oper_r.unwrap();

        let rs = &rs[hash[&oper.index()]];
        let (addr, x, y) = oper.recv(rs).unwrap();

        if addr == 255 {
            if first_255 {
                println!("part1 answer: {}", y);
                first_255 = false;
            }
            last_xy = (x, y);
            continue;
        }

        ss[addr as usize].send((x, y)).unwrap();
    }

    Ok(())
}
