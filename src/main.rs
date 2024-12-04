use std::io::Read;

// mod day01;
// mod day02;
// mod day03;
mod day04;

fn run(day: u32, solve_fn: fn(&str)) {
    let mut input_file =
        std::fs::File::open(format!("inputs/day{day:02}.txt")).expect("Could not open file");
    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .expect("Reading file failed!");

    solve_fn(input.trim());
}

fn main() {
    run(4, day04::solve);
}
