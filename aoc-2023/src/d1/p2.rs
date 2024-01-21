#![allow(dead_code)]

use crate::utils;
use fxhash::FxHashMap;

/// Returns the integer and starting index of the first numeric sequence
/// in the input line, searching from the given start index (inclusive).
/// 
/// # Arguments
/// 
/// * `line`: Input string in which to search
/// * `start_idx`: First index from which to search for a numeric substring
/// 
/// # Returns
/// 
/// * If found, integral form of the number and the starting index within the
/// input string at which the number was found
fn find_number_in_line(line: &str, start_idx: usize) -> Option<(u32, usize)> {
    // Stores the numeric strings to search, mapped to the corresponding
    // integral form, and the number of characters so far matched
    let mut index_map: FxHashMap<&str, (u32, u32)> = [
        ("one", (1, 0)),
        ("two", (2, 0)),
        ("three", (3, 0)),
        ("four", (4, 0)),
        ("five", (5, 0)),
        ("six", (6, 0)),
        ("seven", (7, 0)),
        ("eight", (8, 0)),
        ("nine", (9, 0))
    ].iter().cloned().collect();

    // Take each character in the input line, and check if it is part of a number (integral or string)
    for (char_idx, line_char) in line[start_idx..].as_bytes().iter().enumerate() {
        // If the current character is numeric, we've found the number
        if let Some(num) = utils::bytechar_to_num(*line_char) {
            return Some((num.into(), start_idx + char_idx));
        } else {
            let mut completed_num: Option<&str> = None;

            // Take each number in the index pair, and check if the current character matches the next (or first) character in 
            index_map.iter_mut().for_each(|(numstr, num_status)| {
                let curr_idx = &mut num_status.1;
                // Check if this character is the expected next character to match the current number string
                if (*curr_idx as usize) < numstr.len() && (numstr.as_bytes()[*curr_idx as usize] == *line_char) {
                    *curr_idx += 1;

                    // Fully matched a number
                    if *curr_idx as usize == numstr.len() {
                        completed_num = Some(*numstr);
                    }

                    return;
                }
                
                // Else, check if this character is the first character to match the number strings
                if numstr.as_bytes()[0] == *line_char {
                    *curr_idx = 1;

                    // Fully matched a number
                    if *curr_idx as usize == numstr.len() {
                        completed_num = Some(*numstr);
                    }
                } else {
                    // Reset any progress on matching this numeric string
                    *curr_idx = 0;
                }
            });

            // If we found a number return it
            if let Some(numstr) = completed_num {
                return Some((index_map[numstr].0, start_idx + char_idx  + 1 - numstr.len()));
            }
        }
    }

    None
}

pub(crate) fn run(input_file: &str) -> u32 {

    let mut line_idx = 0;

    return utils::read_file(input_file).map(|line| {
        if let Ok(line) = line {
            // Find first number (from the left) in the given line
            let (first_num, first_num_idx) = find_number_in_line(&line, 0).unwrap_or_else(|| panic!("Could not find a number in string: {}", line));
            
            // Find numbers after index of first number
            let mut last_num = first_num;
            let mut last_idx = first_num_idx;

            loop {
                if let Some((next_num, next_idx)) = find_number_in_line(&line, last_idx + 1) {
                    last_num = next_num;
                    last_idx = next_idx;
                } else {
                    break;
                }
            }
            
            // Build number from retrieved numbers
            let found_num = (10 * first_num) + last_num;
            line_idx += 1;
            
            return found_num;
        }
        
        0
    }).sum::<u32>();
}