use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("File could not be read");
    run(&contents);
    run2(&contents);
}

fn run(input: &String) {
    let mut sum: u32 = 0;
    for sack in input.split("\n") {
        let len = sack.len();
        let (h1, h2) = sack.split_at(len / 2);
        'outer: for i in h1.chars() {
            for j in h2.chars() {
                if i == j {
                    sum += match i {
                        'a'..='z' => i as u32 - 96,
                        'A'..='Z' => i as u32 - 38,
                        _ => 0,
                    };
                    break 'outer;
                }
            }
        }
    }
    println!("{sum}");
}

fn run2(input: &String) {
    let mut sum: u32 = 0;
    let mut count: u8 = 0;
    let mut g1: String = String::new();
    let mut g2: String = String::new();
    for sack in input.split("\n") {
        match count {
            0 => {
                g1 = sack.to_string();
                count += 1;
            }
            1 => {
                g2 = sack.to_string();
                count += 1;
            }
            _ => {
                count = 0;
                'outer: for i in g1.chars() {
                    for j in g2.chars() {
                        for k in sack.to_string().chars() {
                            if i == j && j == k {
                                sum += match i {
                                    'a'..='z' => i as u32 - 96,
                                    'A'..='Z' => i as u32 - 38,
                                    _ => 0,
                                };
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{sum}");
}
