fn check(s: &str) -> bool {
    let mut prev = '0';
    let mut digits = false;
    for c in s.chars() {
        if prev == c {
            digits = true;
        } else if prev > c {
            return false;
        }
        prev = c;
    }
    digits
}

fn main() {
    let mut num = 0;

    for i in 272091..815432 {
        let s = format!("{}", i);
        if check(&s) {
            num += 1
        }
    }

    println!("{}", num);
}
