use cucumber::{gherkin::Step, given, then, World};
use day_1::{parse, sum, CalibrationLines};

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

#[tokio::main]
async fn main() {
    PartOneWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/part1.feature")
        .await;
}
