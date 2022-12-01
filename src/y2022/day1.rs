use std::{cmp::max, collections::BinaryHeap, path::Path};

use crate::utils::read_lines;

pub fn get_max_calories<F>(input_filepath: F) -> Result<i64, &'static str>
where
    F: AsRef<Path>,
{
    let mut curr_total = 0;
    let mut max_calories = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input_line) = line {
                if input_line.trim() == "" {
                    max_calories = max(max_calories, curr_total);
                    curr_total = 0;
                } else {
                    curr_total += input_line.trim().parse().unwrap_or(0);
                }
            }
        }
    } else {
        return Err("Error reading input from input");
    }

    return Ok(max_calories);
}

pub fn get_highest_3_calories<F>(input_filepath: F) -> Result<i64, &'static str>
where
    F: AsRef<Path>,
{
    let mut curr_total = 0;
    let mut elf_calories = BinaryHeap::new();

    let mut max_calories = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input_line) = line {
                if input_line.trim() == "" {
                    elf_calories.push(curr_total);
                    max_calories = max(max_calories, curr_total);
                    curr_total = 0;
                } else {
                    curr_total += input_line.trim().parse().unwrap_or(0);
                }
            }
        }
    } else {
        return Err("Error reading input from input");
    }

    return Ok(elf_calories.pop().unwrap_or(-1)
        + elf_calories.pop().unwrap_or(-1)
        + elf_calories.pop().unwrap_or(-1));
}
