use core::str::Chars;
use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

use crate::utils::read_lines;

fn is_buffer_elements_unique(buffer: &VecDeque<char>) -> bool {
    let mut unique_elements = HashSet::<char>::new();
    let mut curr_uniques = unique_elements.len();

    for element in buffer.iter() {
        unique_elements.insert(*element);
        if unique_elements.len() == curr_uniques {
            return false;
        }

        curr_uniques = unique_elements.len();
    }

    true
}

fn get_start_section_offset_impl(datastream: Chars, buffer_size: usize) -> Option<usize> {
    let mut buffer = VecDeque::with_capacity(buffer_size);
    for (idx, char) in datastream.enumerate() {
        buffer.push_back(char);
        if buffer.len() >= buffer_size {
            if buffer.len() > buffer_size {
                buffer.pop_front();
            }

            if is_buffer_elements_unique(&buffer) {
                return Some(idx + 1);
            }
        }
    }

    None
}

pub fn get_start_marker_offset<F>(input_filepath: F) -> Result<usize, &'static str>
where
    F: AsRef<Path>,
{
    if let Ok(lines) = read_lines(input_filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input_line) = line {
                if let Some(marker_offset) = get_start_section_offset_impl(input_line.chars(), 4) {
                    return Ok(marker_offset);
                }
            }
        }
    } else {
        return Err("Error reading input from input");
    }

    return Err("No start packet marker found");
}

pub fn get_start_message_offset<F>(input_filepath: F) -> Result<usize, &'static str>
where
    F: AsRef<Path>,
{
    if let Ok(lines) = read_lines(input_filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input_line) = line {
                if let Some(marker_offset) = get_start_section_offset_impl(input_line.chars(), 14) {
                    return Ok(marker_offset);
                }
            }
        }
    } else {
        return Err("Error reading input from input");
    }

    return Err("No start message marker found");
}
