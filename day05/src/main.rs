use std::fs;
const STACKS: usize = 9;

fn main() {
    let contents = fs::read_to_string("input").expect("File could not be read");
    run(&contents);
    run2(&contents);
}

fn run(input: &String) {
    let (stacks, instructions) = input.split_at(input.find("\n\n").unwrap());
    let mut stacks: [Vec<char>; STACKS] = make_stacks(stacks);
    instructions.trim().split("\n").for_each(|l| {
        let mut instructions: Vec<usize> = vec![];
        l.split(" ").skip(1).step_by(2).for_each(|x| {
            instructions.push(x.to_string().parse::<usize>().unwrap());
        });

        for _ in 0..instructions[0] {
            let tmp = stacks[instructions[1] - 1].pop().unwrap();
            stacks[instructions[2] - 1].push(tmp);
        }
    });
    print_first_of_all_stacks(&stacks);
}

fn run2(input: &String) {
    let (stacks, instructions) = input.split_at(input.find("\n\n").unwrap());
    let mut stacks: [Vec<char>; STACKS] = make_stacks(stacks);
    instructions.trim().split("\n").for_each(|l| {
        let mut instructions: Vec<usize> = vec![];
        l.split(" ").skip(1).step_by(2).for_each(|x| {
            instructions.push(x.to_string().parse::<usize>().unwrap());
        });

        let mut tmps: Vec<char> = vec![];
        for _ in 0..instructions[0] {
            let tmp = stacks[instructions[1] - 1].pop().unwrap();
            tmps.push(tmp)
        }
        for _ in 0..instructions[0] {
            let tmp = tmps.pop().unwrap();
            stacks[instructions[2] - 1].push(tmp);
        }
    });
    print_first_of_all_stacks(&stacks);
}

fn make_stacks(desc: &str) -> [Vec<char>; STACKS] {
    let mut stacks: [Vec<char>; STACKS] = Default::default();
    let mut i: usize = 0;
    let mut j: usize = 0;
    desc.lines().rev().skip(1).for_each(|l| {
        i = 0;
        j = 0;
        l.chars().skip(1).for_each(|c| {
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
            if j % 4 == 0 {
                i += 1;
            }
            j += 1;
        });
    });
    stacks
}

fn print_first_of_all_stacks(stacks: &[Vec<char>; STACKS]) {
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!("");
}
