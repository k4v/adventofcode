use std::{path::Path, cmp::{min, max}};

use crate::utils::read_lines;


fn is_range_contained(first_range: (u8, u8), second_range: (u8, u8)) -> bool {
    let combined_range = (min(first_range.0, second_range.0), max(first_range.1, second_range.1));
    combined_range == first_range || combined_range == second_range
}

fn do_ranges_overlap(first_range: (u8, u8), second_range: (u8, u8)) -> bool {
    first_range.0 <= second_range.1 && second_range.1 <= first_range.1
        || second_range.0 <= first_range.1 && first_range.1 <= second_range.1
}

pub fn get_fully_contained_pairs<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut num_contained_pairs = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            if let Ok(input_line) = line {
                let mut ranges = vec![];

                for range_str in input_line.split(",") {
                    let range_str_pair = range_str.split("-").collect::<Vec<&str>>();
                    let range: (u8, u8) = (range_str_pair[0].parse().unwrap(), range_str_pair[1].parse().unwrap());

                    ranges.push(range);
                }

                if is_range_contained(ranges[0], ranges[1]) {
                    num_contained_pairs += 1;
                }
            }
        }
    } else {
        return Err("Error reading input from file");
    }

    Ok(num_contained_pairs)
}

pub fn get_total_overlapping_pairs<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut num_contained_pairs = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            if let Ok(input_line) = line {
                let mut ranges = vec![];

                for range_str in input_line.split(",") {
                    let range_str_pair = range_str.split("-").collect::<Vec<&str>>();
                    let range: (u8, u8) = (range_str_pair[0].parse().unwrap(), range_str_pair[1].parse().unwrap());

                    ranges.push(range);
                }

                if do_ranges_overlap(ranges[0], ranges[1]) {
                    num_contained_pairs += 1;
                }
            }
        }
    } else {
        return Err("Error reading input from file");
    }

    Ok(num_contained_pairs)
}

#[cfg(test)]
mod day4_tests {
    use crate::y2022::day4::{is_range_contained, do_ranges_overlap};


    #[test]
    fn test_is_range_contained() {
        assert!(is_range_contained((0, 5), (1, 3)));
        assert!(is_range_contained((0, 3), (0, 5)));
        assert!(is_range_contained((4, 5), (0, 5)));
        assert!(is_range_contained((0, 5), (0, 0)));
        assert!(is_range_contained((0, 5), (5, 5)));

        assert!(!is_range_contained((0, 5), (6, 7)));
        assert!(!is_range_contained((5, 7), (0, 5)));
        assert!(!is_range_contained((0, 5), (3, 6)));
        assert!(!is_range_contained((5, 7), (0, 5)));
    }

    #[test]
    fn test_do_ranges_overlap() {
        assert!(do_ranges_overlap((0, 5), (1, 3)));
        assert!(do_ranges_overlap((5, 7), (0, 5)));
        assert!(do_ranges_overlap((0, 6), (0, 5)));
        assert!(do_ranges_overlap((0, 5), (3, 3)));

        assert!(!do_ranges_overlap((0, 5), (6, 7)));
        assert!(!do_ranges_overlap((5, 7), (2, 4)));
    }
}