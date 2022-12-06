use std::path::Path;

use regex::Regex;

use crate::utils::read_lines;

fn parse_crates_line(crates_line: &String, stacks: &mut Vec<Vec<char>>) -> bool {
    let crate_line_regex = Regex::new(r"(   |\[\w\]) ").expect("Error parsing regex");

    // See if this line corresponds to crates at a particular height
    let crates_matched = crate_line_regex.captures_iter(&crates_line);

    // Which stack to add this current crate
    let mut stack_idx = 0;

    let mut has_match = false;

    for crate_matched in crates_matched.into_iter() {
        let crate_slot = crate_matched[1].trim();
        if !crate_slot.is_empty() {
            while stacks.len() <= stack_idx {
                stacks.push(Vec::new());
            }

            stacks[stack_idx].insert(0, crate_slot.chars().nth(1).unwrap());
        }

        stack_idx += 1;
        has_match = true;
    }

    return has_match;
}

fn do_crates_rearrange(
    current_stacks: &mut Vec<Vec<char>>,
    rearrange_line: &String,
    move_multiple: bool,
) {
    let rearrange_regex =
        Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Error parsing regex");

    // There should only be 1 move sequence captured per input line
    for move_capture in rearrange_regex.captures_iter(&rearrange_line) {
        let mut move_count = move_capture[1].parse::<usize>().unwrap();
        let from_stack = move_capture[2].parse::<usize>().unwrap() - 1;
        let to_stack = move_capture[3].parse::<usize>().unwrap() - 1;

        let start_range = current_stacks[from_stack].len() - move_count;

        let mut moving_crates = vec![];
        for crate_idx in current_stacks[from_stack].iter().rev() {
            moving_crates.push(*crate_idx);

            // If required number of crates have been moved, stop
            move_count -= 1;
            if move_count == 0 {
                break;
            }
        }

        if move_multiple {
            current_stacks[to_stack].extend(&mut moving_crates.iter().rev());
        } else {
            current_stacks[to_stack].extend(&mut moving_crates.iter());
        }
        current_stacks[from_stack].drain(start_range..);
    }
}

pub fn get_stack_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut stack_tops = String::new();
    for stack in stacks.iter() {
        stack_tops.push(*stack.last().unwrap_or(&' '));
    }

    stack_tops
}

pub fn get_crates_on_top_9000<F>(input_filepath: F) -> Result<String, &'static str>
where
    F: AsRef<Path>,
{
    // Store current state of the crate stacks
    let mut stacks = vec![];

    // Whether the input file section with initial state is done parsing
    let mut initial_state_parsed = false;

    // Read each line from the input file
    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            // If an individual line was successfully read
            if let Ok(input_line) = line {
                // If not finished reading initial state, keep updating initial state
                if !initial_state_parsed {
                    initial_state_parsed = !parse_crates_line(&input_line, &mut stacks);
                } else {
                    // Else update stack arrangement
                    do_crates_rearrange(&mut stacks, &input_line, false);
                }
            }
        }

        return Ok(get_stack_tops(&stacks));
    } else {
        eprintln!("Error reading input file");
        return Err("Error reading input file");
    }
}

pub fn get_crates_on_top_9001<F>(input_filepath: F) -> Result<String, &'static str>
where
    F: AsRef<Path>,
{
    // Store current state of the crate stacks
    let mut stacks = vec![];

    // Whether the input file section with initial state is done parsing
    let mut initial_state_parsed = false;

    // Read each line from the input file
    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            // If an individual line was successfully read
            if let Ok(input_line) = line {
                // If not finished reading initial state, keep updating initial state
                if !initial_state_parsed {
                    initial_state_parsed = !parse_crates_line(&input_line, &mut stacks);
                } else {
                    // Else update stack arrangement
                    do_crates_rearrange(&mut stacks, &input_line, true);
                }
            }
        }

        return Ok(get_stack_tops(&stacks));
    } else {
        eprintln!("Error reading input file");
        return Err("Error reading input file");
    }
}
