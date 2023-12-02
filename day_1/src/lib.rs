pub type CalibrationLine = String;
pub type CalibrationLines = Vec<CalibrationLine>;

pub fn parse(line: &CalibrationLine) -> usize {
    let first_number = line.chars().find(|x| x.is_numeric()).unwrap_or('0');
    let last_number = line.chars().rev().find(|x| x.is_numeric()).unwrap_or('0');
    format!("{first_number}{last_number}").parse().unwrap_or(0)
}

pub fn sum(lines: &CalibrationLines) -> usize {
    lines.iter().map(parse).sum()
}
