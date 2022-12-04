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
    // Day 1
    print_result(2022, 1, 1, y2022::day1::get_max_calories("input/2022_d1.input"));
    print_result(2022, 1, 2, y2022::day1::get_highest_3_calories("input/2022_d1.input"));

    // Day 2
    print_result(2022, 2, 1, y2022::day2::get_rps_score_by_moves("input/2022_d2.input"));
    print_result(2022, 2, 2, y2022::day2::get_rps_score_by_result("input/2022_d2.input"));

    // Day 3
    print_result(2022, 3, 1, y2022::day3::get_duplicate_items_priority_total("input/2022_d3.input"));
    print_result(2022, 3, 2, y2022::day3::get_badge_priority_total("input/2022_d3.input"));
}
