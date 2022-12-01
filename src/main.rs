use std::fmt::Debug;

mod utils;
mod y2022;

fn print_result<T>(year: u64, day_idx: u64, puzzle_idx: u64, result: Result<T, &'static str>)
where
    T: Debug,
{
    if let Ok(solution) = result {
        println!("Y{year}/D{day_idx}/P{puzzle_idx}: {:?}", solution);
    } else {
        println!("Y{year}/D{day_idx}/P{puzzle_idx}: {:?}", result.err().unwrap());
    }
}

fn main() {
    print_result(2022, 1, 1, y2022::day1::get_max_calories("input/2022_d1.input"));
    print_result(2022, 1, 2, y2022::day1::get_highest_3_calories("input/2022_d1.input"));
}
