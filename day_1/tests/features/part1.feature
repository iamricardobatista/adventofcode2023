# Advent of code: https://adventofcode.com/2023/day/1
Feature: Calibration document
    Scenario: Part 1: Calculate the sum of all calibration values
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
    Scenario: Part 2: Some digits are actually spelled out with letters
        Given a set of calibration lines
            | two1nine          |
            | eightwothree      |
            | abcone2threexyz   |
            | xtwone3four       |
            | 4nineeightseven2  |
            | zoneight234       |
            | 7pqrstsixteen     |
        Then the calibration with numbers writen in full values are
            | 29 |
            | 83 |
            | 13 |
            | 24 |
            | 42 |
            | 14 |
            | 76 |
        And the sum in full of all the calibration values is 281
