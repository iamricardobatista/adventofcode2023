use std::{env, fs::read_to_string};

use day_1::{sum, CalibrationLines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let calibration_lines: CalibrationLines = read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    print!(
        "The sum of all calibration values is: {:?}",
        sum(&calibration_lines)
    );
}
