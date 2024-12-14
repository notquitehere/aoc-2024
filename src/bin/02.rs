use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(2);

fn process_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in input.split("\n") {
        reports.push(line.split_whitespace().map(|x| i32::from_str(x).unwrap()).collect::<Vec<i32>>());
    }
    reports
}

fn check_report(report: &Vec<i32>) -> bool {
    if !(report.is_sorted() || report.is_sorted_by(|a,b| b <= a)){
        return false;
    }

    let end = report.len()-1;

    for i in 0..end {
        let section = &report[i..i+2];

        let diff = (section[1] - section[0]).abs();
        if diff == 0  || diff > 3 {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = process_input(input);
    let mut total = 0;

    for report in reports {
        if check_report(&report) {
            total += 1;
        }
    }
    Some(total)
}


pub fn part_two(input: &str) -> Option<u32> {
    let reports = process_input(input);
    let mut safe_reports = 0;

    for report in reports {
        if check_report(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut temp_report = report.to_vec();
                temp_report.remove(i);
                if check_report(&temp_report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
