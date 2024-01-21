#![allow(dead_code)]

use crate::utils;

pub(crate) fn run(input_file: &str) -> u32 {
    return utils::read_file(input_file).map(|line| {

        // Calibration value in this line
        let mut calib_value = 0;

        let line = line.unwrap();
        let bytes = line.as_bytes();

        // Get the first digit in forward direction
        for i in 0 .. bytes.len() {
            if let Some(num) = utils::bytechar_to_num(bytes[i]) {
                calib_value = num;
                break;
            }
        }

        for i in 0 .. bytes.len() {
            let idx = bytes.len() - i - 1;
            if let Some(num) = utils::bytechar_to_num(bytes[idx]) {
                calib_value = (calib_value * 10) + num;
                break;
            }
        }

        Into::<u32>::into(calib_value)
    }).sum::<u32>();
    
}