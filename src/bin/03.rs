use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(3);

fn process_input(input: &str) -> String {
    let mut program: String = "".to_owned();

    for line in input.split("\n") {
        program.push_str(line);
    }
    program
}

fn calculate(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut ans = 0;

    for (_, [ a, b ]) in re.captures_iter(&input).map(|c| c.extract()) {
        ans +=  i32::from_str(a).unwrap() * i32::from_str(b).unwrap();
    };
    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    let program = process_input(input);

    Some(calculate(&program).try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let binding = process_input(input);
    let program: Vec<&str> = binding.split("do()").collect();
    let mut ans = 0;

    for line in program {
        let enabled = line.split("don't()").collect::<Vec<&str>>()[0];
        ans += calculate(&enabled);
    }
    Some(ans.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
