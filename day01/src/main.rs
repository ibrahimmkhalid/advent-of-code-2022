use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("File could not be read");
    run(&contents);
    run2(contents);
}

fn run(input: &String) {
    let mut sum: i32 = 0;
    let mut max: i32 = 0;
    for elf in input.trim().split("\n\n") {
        for calories in elf.split("\n") {
            let calories: i32 = calories.parse().expect("unable to read number");
            sum = sum + calories
        }
        if sum > max {
            max = sum;
        }
        sum = 0;
    }
    println!("{max}");
}

fn run2(input: String) {
    let mut sum: i32 = 0;
    let mut totals: Vec<i32> = Vec::new();
    for elf in input.trim().split("\n\n") {
        for calories in elf.split("\n") {
            let calories: i32 = calories.parse().expect("unable to read number");
            sum = sum + calories
        }
        totals.push(sum);
        sum = 0;
    }

    let len: usize = totals.len();
    qsort(&mut totals, 0, len - 1);
    totals.reverse();
    let mut three_sum: i32 = 0;
    for x in 0..3 {
        let y: i32 = totals[x];
        three_sum = y + three_sum;
    }
    println!("{three_sum}");
}

fn qsort(arr: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo < hi {
        let pivot_idx: usize = partition(arr, lo, hi);
        if pivot_idx >= 1 {
            qsort(arr, lo, pivot_idx);
        }
        qsort(arr, pivot_idx + 1, hi);
    }
}

fn partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    let pivot_idx: usize = (hi + lo) / 2;
    let pivot: i32 = arr[pivot_idx];

    let mut left: usize = lo;
    let mut right: usize = hi;

    loop {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }

        if left >= right {
            break;
        }

        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
    right
}
