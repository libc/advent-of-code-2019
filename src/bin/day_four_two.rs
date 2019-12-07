fn check(s: &str) -> bool {
    let mut prev = '0';
    let mut run = 0;
    let mut seen_run = false;

    for c in s.chars() {
        if prev == c {
            run += 1;
        } else {
            if run == 1 {
                seen_run = true;
            }

            run = 0;
            if prev > c {
                return false;
            }
        }
        prev = c;
    }
    println!("{}", run);
    if run == 1 {
        seen_run = true;
    }
    seen_run
}

fn main() {
    let mut num = 0;

    for i in 272091..815432 {
        let s = format!("{}", i);
        if check(&s) {
            num += 1
        }
    }

    println!("{}", check("123444"));
    println!("{}", check("112233"));
    println!("{}", check("111122"));
    println!("{}", num);
}
