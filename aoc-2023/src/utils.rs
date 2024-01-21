use std::io::{BufReader, BufRead, Lines};
use std::fs::File;

pub(crate) fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file_reader = BufReader::new(File::open(file_name).expect("Cannot open input file"));

    return file_reader.lines();
}

pub(crate) fn bytechar_to_num(byte: u8) -> Option<u8> {
    if byte >= 48 && byte <= 57 {
        return Some(byte - 48);
    } else {
        return None
    }

}