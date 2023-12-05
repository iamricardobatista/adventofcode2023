use cucumber::{gherkin::Step, given, then, World};
use day_1::{parse, parse_in_full, sum, sum_in_full, CalibrationLines};

#[derive(Debug, Default, World)]
struct PartOneWorld {
    lines: CalibrationLines,
}

#[given(expr = "a set of calibration lines")]
fn given_a_set_of_calibration_lines(world: &mut PartOneWorld, step: &Step) {
    if let Some(table) = step.table().as_ref() {
        world.lines = table.rows.iter().map(|row| row[0].clone()).collect();
    }
}

#[then(expr = "the calibration with numbers writen in full values are")]
fn then_the_calibration_with_numbers_in_fullvalues_are(world: &mut PartOneWorld, step: &Step) {
    let calibratio_values: Vec<usize> = world.lines.iter().map(parse_in_full).collect();
    if let Some(table) = step.table.as_ref() {
        let expected_values: Vec<usize> = table
            .rows
            .iter()
            .map(|row| row[0].parse().unwrap_or(0))
            .collect();

        assert_eq!(calibratio_values, expected_values);
    }
}

#[then(expr = "the calibration values are")]
fn then_the_calibration_values_are(world: &mut PartOneWorld, step: &Step) {
    let calibratio_values: Vec<usize> = world.lines.iter().map(parse).collect();
    if let Some(table) = step.table.as_ref() {
        let expected_values: Vec<usize> = table
            .rows
            .iter()
            .map(|row| row[0].parse().unwrap_or(0))
            .collect();

        assert_eq!(calibratio_values, expected_values);
    }
}

#[then(expr = "the sum of all the calibration values is {int}")]
fn then_the_sum_of_all_the_calibration_values_is(world: &mut PartOneWorld, number: usize) {
    assert_eq!(sum(&world.lines), number);
}

#[then(expr = "the sum in full of all the calibration values is {int}")]
fn then_the_sum_in_full_of_all_the_calibration_values_is(world: &mut PartOneWorld, number: usize) {
    assert_eq!(sum_in_full(&world.lines), number);
}
#[tokio::main]
async fn main() {
    PartOneWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/part1.feature")
        .await;
}
