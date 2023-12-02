# Advent of code: https://adventofcode.com/2023/day/1
Feature: Calibration document
    Scenario: Calculate the sum of all calibration values
        Given a set of calibration lines
            | 1abc2         |
            | pqr3stu8vwx   |
            | a1b2c3d4e5f   |
            | treb7uchet    |
        Then the calibration values are
            | 12 |
            | 38 |
            | 15 |
            | 77 |
        And the sum of all the calibration values is 142
