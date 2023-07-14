use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("File could not be read");
    run(&contents);
    run2(&contents);
}

/*
 * input example
 * 2-3,4-8
 */

fn run(input: &String) {
    let count = input
        .lines()
        .map(|pair| {
            let (left, right) = pair.split_once(",").unwrap();
            let ((a, b), (c, d)) = (
                left.split_once("-").unwrap(),
                right.split_once("-").unwrap(),
            );
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count();

    println!("{count}");
}

fn run2(input: &String) {
    let count = input
        .lines()
        .map(|pair| {
            let (left, right) = pair.split_once(",").unwrap();
            let ((a, b), (c, d)) = (
                left.split_once("-").unwrap(),
                right.split_once("-").unwrap(),
            );
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| {
            (a <= d && b >= d)
                || (a <= c && b >= c)
                || (c <= a && d >= a)
                || (c <= b && d >= b)
                || (a >= c && b <= d)
                || (a <= c && b >= d)
        })
        .count();

    println!("{count}");
}
