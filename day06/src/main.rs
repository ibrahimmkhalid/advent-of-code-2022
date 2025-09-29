use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("File could not be read");
    run(&contents);
    // run2(&contents);
}


fn run(input: &String) {
    let input: Vec<char> = input.into();
}

// fn run2(input: &String) {
//
// }
