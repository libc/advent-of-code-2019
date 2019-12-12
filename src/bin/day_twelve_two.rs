struct Object {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Object {
    fn new(x: i64, y: i64, z: i64) -> Object {
        Object {
            x: x,
            y: y,
            z: z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }
}

fn delta(a: i64, b: i64) -> i64 {
    if a > b {
        -1
    } else if a < b {
        1
    } else {
        0
    }
}

fn simulation_step(obj: &mut Vec<Object>) {
    for i in 0..obj.len() {
        for j in 0..obj.len() {
            if i == j {
                continue;
            }

            obj[i].vx += delta(obj[i].x, obj[j].x);
            obj[i].vy += delta(obj[i].y, obj[j].y);
            obj[i].vz += delta(obj[i].z, obj[j].z);
        }
    }

    for i in 0..obj.len() {
        obj[i].x += obj[i].vx;
        obj[i].y += obj[i].vy;
        obj[i].z += obj[i].vz;
    }
}

fn test1() -> Vec<Object> {
    vec![
        Object::new(-1, 0, 2),
        Object::new(2, -10, -7),
        Object::new(4, -8, 8),
        Object::new(3, 5, -1),
    ]
}

fn task() -> Vec<Object> {
    vec![
        Object::new(-16, -1, -12),
        Object::new(0, -4, -17),
        Object::new(-11, 11, 0),
        Object::new(2, 2, -6),
    ]
}

fn find_cycle<F>(mut obj: Vec<Object>, f: F) -> usize
where
    F: Fn(&Vec<Object>) -> i64,
{
    let mut v = Vec::new();

    loop {
        simulation_step(&mut obj);
        v.push(f(&obj));

        if v.len() % 3 == 0 {
            let s = v.len() / 3;
            let mut m = true;
            for j in 0..s {
                if v[j] != v[j + s] || v[j] != v[j + s * 2] {
                    m = false;
                    break;
                }
            }
            if m {
                return s;
            }
        }
    }
}

fn primes(n: usize) -> Vec<usize> {
    let mut p = vec![2, 3, 5, 7];

    for i in 11..n {
        if p.iter().any(|pp| i % pp == 0) {
            continue;
        }
        p.push(i);
    }

    p
}

fn lcm(n: Vec<usize>) -> usize {
    if n.is_empty() {
        return 0;
    }
    let p = primes(*n.iter().max().unwrap());

    let m = n
        .iter()
        .map(|i| {
            let ii = *i;
            p.iter()
                .map(move |pp| {
                    let mut j = 1;
                    let mut k = ii;
                    while k % *pp == 0 {
                        j *= *pp;
                        k /= *pp;
                    }
                    j
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut output = 1;

    for i in 0..p.len() {
        output *= m.iter().map(|mm| (*mm)[i]).max().unwrap();
    }

    output
}

fn main() {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();

    let mut o = test1();

    for _ in 0..=100 {
        simulation_step(&mut o);
        xs.push(o[0].x);
        ys.push(o[0].y);
        zs.push(o[0].z);
    }
    println!("xs: {:?}", xs);
    println!("ys: {:?}", ys);
    println!("zs: {:?}", zs);

    let cycles = vec![
        find_cycle(test1(), |obj| obj[0].x),
        find_cycle(test1(), |obj| obj[0].y),
        find_cycle(test1(), |obj| obj[0].z),
        find_cycle(test1(), |obj| obj[1].x),
        find_cycle(test1(), |obj| obj[1].y),
        find_cycle(test1(), |obj| obj[1].z),
        find_cycle(test1(), |obj| obj[2].x),
        find_cycle(test1(), |obj| obj[2].y),
        find_cycle(test1(), |obj| obj[2].z),
        find_cycle(test1(), |obj| obj[3].x),
        find_cycle(test1(), |obj| obj[3].y),
        find_cycle(test1(), |obj| obj[3].z),
    ];

    println!("{:?}", cycles);
    println!("{}", lcm(cycles));

    let mut cycles = Vec::new();
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[0].x));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[0].y));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[0].z));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[1].x));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[1].y));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[1].z));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[2].x));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[2].y));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[2].z));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[3].x));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[3].y));
    println!("#");
    cycles.push(find_cycle(task(), |obj| obj[3].z));
    println!("");

    println!("{:?}", cycles);
    println!("{}", lcm(cycles));
}
