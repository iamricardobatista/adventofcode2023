use std::{env, fs::read_to_string};

use day_1::{sum, sum_in_full, CalibrationLines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let calibration_lines: CalibrationLines = read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!(
        "The sum of all calibration values is: {:?}",
        sum(&calibration_lines)
    );

    println!(
        "The sum of all calibration in full values is: {:?}",
        sum_in_full(&calibration_lines)
    );
}
