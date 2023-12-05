use regex::Regex;

pub type CalibrationLine = String;
pub type CalibrationLines = Vec<CalibrationLine>;

pub fn parse(line: &CalibrationLine) -> usize {
    let first_number = line.chars().find(|x| x.is_numeric()).unwrap_or('0');
    let last_number = line.chars().rev().find(|x| x.is_numeric()).unwrap_or('0');
    format!("{first_number}{last_number}").parse().unwrap_or(0)
}

fn match_word(word: &str) -> usize {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.parse().unwrap_or(0),
    }
}

pub fn parse_in_full(line: &CalibrationLine) -> usize {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let results: Vec<usize> = re.find_iter(line).map(|m| match_word(m.as_str())).collect();
    let first_number = results.first().unwrap_or(&0);
    let last_number = results.last().unwrap_or(&0);
    format!("{first_number}{last_number}").parse().unwrap_or(0)
}

pub fn sum(lines: &CalibrationLines) -> usize {
    lines.iter().map(parse).sum()
}

pub fn sum_in_full(lines: &CalibrationLines) -> usize {
    lines.iter().map(parse_in_full).sum()
}
