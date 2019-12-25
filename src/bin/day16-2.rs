use std::iter::repeat;

fn phases(input: &str, n: usize) -> String {
    let mut v = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect::<Vec<i8>>();

    for _ in 0..n {
        for j in (1..input.len()).rev() {
            v[j - 1] = (v[j] + v[j - 1]) % 10;
        }
    }

    v.iter()
        .map(|d| format!("{}", d))
        .collect::<Vec<String>>()
        .join("")
}

fn phases_skip(input: &str, n: usize) -> String {
    let output = phases(
        &repeat(input)
            .map(|s| String::from(s))
            .take(10000)
            .collect::<Vec<String>>()
            .join(""),
        n,
    );

    let skip = input.get(0..7).unwrap().parse::<usize>().unwrap() % output.len();

    String::from(output.get(skip..(skip + 8)).unwrap())
}

fn main() {
    println!("{}", phases("12345678", 1));
    println!("{}", phases("12345678", 2));
    println!("{}", phases("12345678", 3));
    println!("{}", phases("12345678", 4));
    // println!(
    //     "{}",
    //     phases("80871224585914546619083218645595", 100)
    //         .get(0..8)
    //         .unwrap()
    // );
    // println!(
    //     "{}",
    //     phases("19617804207202209144916044189917", 100)
    //         .get(0..8)
    //         .unwrap()
    // );
    // println!(
    //     "{}",
    //     phases("69317163492948606335995924319873", 100)
    //         .get(0..8)
    //         .unwrap()
    // );

    // let task = "59776034095811644545367793179989602140948714406234694972894485066523525742503986771912019032922788494900655855458086979764617375580802558963587025784918882219610831940992399201782385674223284411499237619800193879768668210162176394607502218602633153772062973149533650562554942574593878073238232563649673858167635378695190356159796342204759393156294658366279922734213385144895116649768185966866202413314939692174223210484933678866478944104978890019728562001417746656699281992028356004888860103805472866615243544781377748654471750560830099048747570925902575765054898899512303917159138097375338444610809891667094051108359134017128028174230720398965960712";
    // println!("{}", phases(task, 100).get(0..8).unwrap());

    // println!("");
    // println!("part 2:");

    // println!("{}", phases_skip("80871224585914546619083218645595", 100));
    // println!("{}", phases_skip("19617804207202209144916044189917", 100));
    // println!("{}", phases_skip("69317163492948606335995924319873", 100));

    let task = "59776034095811644545367793179989602140948714406234694972894485066523525742503986771912019032922788494900655855458086979764617375580802558963587025784918882219610831940992399201782385674223284411499237619800193879768668210162176394607502218602633153772062973149533650562554942574593878073238232563649673858167635378695190356159796342204759393156294658366279922734213385144895116649768185966866202413314939692174223210484933678866478944104978890019728562001417746656699281992028356004888860103805472866615243544781377748654471750560830099048747570925902575765054898899512303917159138097375338444610809891667094051108359134017128028174230720398965960712";
    println!("{}", phases_skip(task, 100));
}
