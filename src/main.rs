use std::io::{self, BufRead};

use rootcause::{Report, prelude::ResultExt, report};

fn part_1(input: &str) -> Result<i32, Report> {
    let nums = input
        .split('\n')
        .map(|l| {
            match l
                .chars()
                .next()
                .ok_or_else(|| report!("Could not get first character of line"))?
            {
                'L' => Ok(-l[1..].parse::<i32>()?),
                'R' => Ok(l[1..].parse::<i32>()?),
                _ => Err(report!("First character not valid")),
            }
        })
        .collect::<Result<Vec<i32>, Report>>()?;

    Ok(nums
        .iter()
        .fold((50, 0), |a, b| {
            let mut sum = a.0 + b;

            let result = loop {
                if sum <= 0 {
                    sum += 100;
                } else if sum > 99 {
                    sum -= 100;
                }

                if (0..=99).contains(&sum) {
                    break sum;
                }
            };

            if result == 0 {
                (result, a.1 + 1)
            } else {
                (result, a.1)
            }
        })
        .1)
}

fn part_2(input: &str) -> Result<i32, Report> {
    let nums = input
        .split('\n')
        .map(|l| {
            match l
                .chars()
                .next()
                .ok_or_else(|| report!("Could not get first character of line"))?
            {
                'L' => Ok(-l[1..].parse::<i32>()?),
                'R' => Ok(l[1..].parse::<i32>()?),
                _ => Err(report!("First character not valid")),
            }
        })
        .collect::<Result<Vec<i32>, Report>>()?;

    Ok(nums
        .iter()
        .fold((50, 0), |a, b| {
            let mut sum = a.0 + b;
            let mut clicks = a.1;

            let mut first_iter = true;
            let result = loop {
                if sum < 0 {
                    if sum != 0 && !(first_iter && a.0 == 0) {
                        println!("Clicked over when rotating {b}");
                        clicks += 1;
                    }

                    sum += 100;
                } else if sum > 99 {
                    if sum != 100 {
                        println!("Clicked over when rotating {b}");
                        clicks += 1;
                    }
                    sum -= 100;
                }

                first_iter = false;

                if (0..=99).contains(&sum) {
                    break sum;
                }
            };

            if result == 0 {
                println!("Landed on 0");
                clicks += 1;
            }

            println!("Movement = {b}");
            println!("Total = {result}");
            println!("Clicks = {clicks}\n");

            (result, clicks)
        })
        .1)
}

fn main() -> Result<(), Report> {
    let stdin = io::stdin();
    let mut input = String::new();

    for line in stdin.lock().lines() {
        input.push_str(&line.context("Could not read stdin")?);
        input.push('\n');
    }

    input.pop();

    println!("{}", part_2(input.as_str())?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_1(input).expect("Error while calculating result"), 3);
    }

    #[test]
    fn test_part_2_part_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_2(input).expect("Error while calculating result"), 6);
    }

    #[test]
    fn test_part_2_part_2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
L361
R14
L82";

        assert_eq!(part_2(input).expect("Error while calculating result"), 9);
    }
}
