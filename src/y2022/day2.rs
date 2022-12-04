use std::path::Path;

use crate::utils::read_lines;

const SCORE_MULTIPLIER: u8 = 3;

#[repr(u8)]
enum RpsResult {
    Lose = 0,
    Draw = 1,
    Win = 2,
}

impl RpsResult {
    fn from(num_repr: u8) -> Option<RpsResult> {
        if num_repr == 0 {
            return Some(RpsResult::Lose);
        }

        if num_repr == 1 {
            return Some(RpsResult::Draw);
        }

        if num_repr == 2 {
            return Some(RpsResult::Win);
        }

        None
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum RpsMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RpsMove {
    fn from(num_repr: u8) -> Option<RpsMove> {
        if num_repr == 1 {
            return Some(RpsMove::Rock);
        }

        if num_repr == 2 {
            return Some(RpsMove::Paper);
        }

        if num_repr == 3 {
            return Some(RpsMove::Scissors);
        }

        None
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        static NUM_MOVE_OPTIONS: u8 = 3;

        let self_value = *self as u8;
        let other_value = *other as u8;

        if self_value == other_value {
            std::cmp::Ordering::Equal
        } else if self_value == other_value + 1
            || self_value == (other_value + 1) % NUM_MOVE_OPTIONS
        {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

fn convert_to_int(of_c: char, base: char) -> u8 {
    (of_c as u8) - (base as u8)
}

fn get_score_from_moves(my_move: &RpsMove, opp_move: &RpsMove) -> u8 {
    let result: RpsResult = match my_move.cmp(opp_move) {
        std::cmp::Ordering::Equal => RpsResult::Draw,
        std::cmp::Ordering::Less => RpsResult::Lose,
        std::cmp::Ordering::Greater => RpsResult::Win,
    };

    (result as u8 * SCORE_MULTIPLIER) + (*my_move as u8)
}

pub fn get_rps_score_by_moves<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut total_score = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            if let Ok(input_line) = line {
                let moves = input_line.split_ascii_whitespace().collect::<Vec<&str>>();
                let opp_move =
                    RpsMove::from(1 + convert_to_int(moves[0].chars().nth(0).unwrap(), 'A'))
                        .unwrap();
                let my_move =
                    RpsMove::from(1 + convert_to_int(moves[1].chars().nth(0).unwrap(), 'X'))
                        .unwrap();

                let score = get_score_from_moves(&my_move, &opp_move);
                total_score += score as u64;
            }
        }
    } else {
        return Err("Error reading input from file");
    }

    Ok(total_score)
}

fn get_move_by_result(opp_move: &RpsMove, result: &RpsResult) -> RpsMove {
    match result {
        RpsResult::Lose => {
            let my_move = (*opp_move as u8) - 1;
            RpsMove::from(if my_move == 0 { 3 } else { my_move }).unwrap()
        }
        RpsResult::Draw => *opp_move,
        RpsResult::Win => RpsMove::from((*opp_move as u8 % 3) + 1).unwrap(),
    }
}

pub fn get_rps_score_by_result<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut total_score = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            if let Ok(input_line) = line {
                let moves = input_line.split_ascii_whitespace().collect::<Vec<&str>>();
                let opp_move =
                    RpsMove::from(1 + convert_to_int(moves[0].chars().nth(0).unwrap(), 'A'))
                        .unwrap();
                let result =
                    RpsResult::from(convert_to_int(moves[1].chars().nth(0).unwrap(), 'X')).unwrap();
                let my_move = get_move_by_result(&opp_move, &result);

                let score = get_score_from_moves(&my_move, &opp_move);
                total_score += score as u64;
            }
        }
    } else {
        return Err("Error reading input from file");
    }

    Ok(total_score)
}

#[cfg(test)]
mod day2_tests {
    use crate::y2022::day2::RpsMove;

    use super::{convert_to_int, get_score_from_moves};

    #[test]
    fn test_convert_to_int() {
        assert_eq!(convert_to_int('A', 'A'), 1);
        assert_eq!(convert_to_int('B', 'A'), 2);
        assert_eq!(convert_to_int('C', 'A'), 3);

        assert_eq!(convert_to_int('X', 'X'), 1);
        assert_eq!(convert_to_int('Y', 'X'), 2);
        assert_eq!(convert_to_int('Z', 'X'), 3);
    }

    #[test]
    fn test_get_my_score() {
        assert_eq!(get_score_from_moves(&RpsMove::Rock, &RpsMove::Rock), 4);
        assert_eq!(get_score_from_moves(&RpsMove::Rock, &RpsMove::Paper), 1);
        assert_eq!(get_score_from_moves(&RpsMove::Rock, &RpsMove::Scissors), 7);

        assert_eq!(get_score_from_moves(&RpsMove::Paper, &RpsMove::Rock), 8);
        assert_eq!(get_score_from_moves(&RpsMove::Paper, &RpsMove::Paper), 5);
        assert_eq!(get_score_from_moves(&RpsMove::Paper, &RpsMove::Scissors), 2);

        assert_eq!(get_score_from_moves(&RpsMove::Scissors, &RpsMove::Rock), 3);
        assert_eq!(get_score_from_moves(&RpsMove::Scissors, &RpsMove::Paper), 9);
        assert_eq!(get_score_from_moves(&RpsMove::Scissors, &RpsMove::Scissors), 6);
    }
}
