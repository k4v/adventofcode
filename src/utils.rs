use std::io::{self, BufRead, BufReader, Lines};
use std::{fs::File, path::Path};

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn convert_char_to_u8(of_c: char, base: char) -> u8 {
    (of_c as u8) - (base as u8)
}
