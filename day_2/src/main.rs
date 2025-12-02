use std::{
    io::{self, BufRead},
    ops::Range,
};

use rootcause::{
    Report,
    prelude::{IteratorExt, ResultExt},
    report,
};

#[allow(dead_code)]
fn part_1(input: &str) -> Result<i64, Report> {
    Ok(input
        .split(',')
        .map(|range| -> Result<Range<i64>, Report> {
            let split = range
                .split_once('-')
                .ok_or_else(|| report!("Could not split on '-'").attach(range.to_owned()))?;

            Ok(split.0.parse()?..split.1.parse::<i64>()? + 1)
        })
        .collect_reports::<Vec<Range<i64>>, _>()
        .map_err(|errors| errors.context("Parsing failed").into_dyn_any())?
        .into_iter()
        .flatten()
        .filter(|num| {
            let num_string = num.to_owned().to_string();
            let length = num_string.chars().count();
            let split = num_string.split_at(length / 2);
            *split.0 == *split.1
        })
        .sum())
}

fn part_2(input: &str) -> Result<i64, Report> {
    Ok(input
        .split(',')
        .map(|range| -> Result<Range<i64>, Report> {
            let split = range
                .split_once('-')
                .ok_or_else(|| report!("Could not split on '-'").attach(range.to_owned()))?;

            Ok(split.0.parse()?..split.1.parse::<i64>()? + 1)
        })
        .collect_reports::<Vec<Range<i64>>, _>()
        .map_err(|errors| errors.context("Parsing failed").into_dyn_any())?
        .into_iter()
        .flatten()
        .filter(|num| {
            let num_string = num.to_owned().to_string();
            let length = num_string.chars().count();
            (0..=length / 2)
                .filter(|i| length.is_multiple_of(*i))
                .filter(|n| {
                    let parts: Vec<String> = num_string
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(*n)
                        .map(|s| s.iter().collect::<String>())
                        .collect();

                    let head = parts
                        .first()
                        .expect("Could not get first item from numbers");

                    parts.iter().all(|s| *s == *head)
                })
                .count()
                >= 1
        })
        .sum())
}

fn main() -> Result<(), Report> {
    let stdin = io::stdin();
    let mut input = String::new();

    for line in stdin.lock().lines() {
        input.push_str(&line.context("Could not read stdin")?);
        input.push('\n');
    }

    input.pop();

    println!("Part 1 = {}", part_1(input.as_str())?);
    println!("Part 2 = {}", part_2(input.as_str())?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part_1_example() {
        assert_eq!(
            part_1(INPUT).expect("Error while calculating result"),
            1_227_775_554
        );
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(INPUT).expect("Error while calculating result"),
            4_174_379_265
        );
    }
}
