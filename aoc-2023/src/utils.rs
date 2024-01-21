use std::io::{BufReader, BufRead, Lines};
use std::fs::File;

pub(crate) fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(file_name).expect("Cannot open input file")).lines()
}

pub(crate) fn bytechar_to_num(byte: u8) -> Option<u8> {
    if (48..=57).contains(&byte) { Some(byte-48) } else { None }
}