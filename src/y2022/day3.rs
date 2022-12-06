use std::{collections::HashSet, path::Path};

use crate::utils::{decode_char_to_u8, read_lines};

pub fn get_duplicate_items_priority_total<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut total_score = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        for line in lines {
            if let Ok(input_line) = line {
                let first_compartment = &input_line[0..input_line.len() / 2];
                let second_compartment = &input_line[input_line.len() / 2..input_line.len()];

                let mut first_char_list: HashSet<char> = HashSet::new();
                let mut second_char_list: HashSet<char> = HashSet::new();

                for char in first_compartment.chars() {
                    first_char_list.insert(char);
                }

                for char in second_compartment.chars() {
                    second_char_list.insert(char);
                }

                for char in first_char_list.intersection(&second_char_list) {
                    total_score += 1
                        + (decode_char_to_u8(char.to_ascii_lowercase(), 'a')
                            + (if char.is_ascii_uppercase() { 26 } else { 0 }))
                            as u64;
                }
            }
        }
    } else {
        return Err("Error reading input from file");
    }

    Ok(total_score)
}

pub fn get_badge_priority_total<F>(input_filepath: F) -> Result<u64, &'static str>
where
    F: AsRef<Path>,
{
    let mut total_score = 0;

    if let Ok(lines) = read_lines(input_filepath) {
        let mut badge_group: Vec<String> = vec![];
        for line in lines {
            if let Ok(input_line) = line {
                badge_group.push(input_line);

                if badge_group.len() == 3 {
                    let badge_entry_set = HashSet::<_>::from_iter(badge_group[0].chars());
                    for char in HashSet::<_>::from_iter(badge_group[1].chars())
                        .intersection(&HashSet::<_>::from_iter(badge_group[2].chars()))
                    {
                        if badge_entry_set.contains(&char) {
                            total_score += 1
                                + (decode_char_to_u8(char.to_ascii_lowercase(), 'a')
                                    + (if char.is_ascii_uppercase() { 26 } else { 0 }))
                                    as u64;
                        }
                    }

                    badge_group.clear();
                }
            }
        }
    }

    Ok(total_score)
}
