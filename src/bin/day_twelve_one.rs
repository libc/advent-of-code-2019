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

fn print(obj: &Vec<Object>) {
    for i in 0..obj.len() {
        println!(
            "pos=<{}, {}, {}>, vel=<{}, {}, {}>",
            obj[i].x, obj[i].y, obj[i].z, obj[i].vx, obj[i].vy, obj[i].vz
        );
    }
}

fn total_energy(obj: &Vec<Object>) -> i64 {
    obj.iter()
        .map(|o| (o.x.abs() + o.y.abs() + o.z.abs()) * (o.vx.abs() + o.vy.abs() + o.vz.abs()))
        .sum()
}

fn main() {
    let mut o = vec![
        Object::new(-1, 0, 2),
        Object::new(2, -10, -7),
        Object::new(4, -8, 8),
        Object::new(3, 5, -1),
    ];

    for i in 0..10 {
        println!("iteration {}", i);
        simulation_step(&mut o);
        print(&o);
    }

    println!("total energy: {}", total_energy(&o));

    let mut o = vec![
        Object::new(-16, -1, -12),
        Object::new(0, -4, -17),
        Object::new(-11, 11, 0),
        Object::new(2, 2, -6),
    ];

    for i in 0..1000 {
        println!("iteration {}", i);
        simulation_step(&mut o);
        print(&o);
    }

    println!("total energy: {}", total_energy(&o));
}
