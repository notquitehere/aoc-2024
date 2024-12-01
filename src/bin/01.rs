use std::str::FromStr;
advent_of_code::solution!(1);

fn process_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in lines {
        let ab = line.split_whitespace().collect::<Vec<&str>>();
        list1.push(i32::from_str(ab[0]).unwrap());
        list2.push(i32::from_str(ab[1]).unwrap());
    }
    (list1, list2)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut list1, mut list2) = process_input(input);
    list1.sort();
    list2.sort();

    let mut total = 0;

    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (list1, list2) = process_input(input);
    let mut total: i32 = 0;

    for val in list1 {
        let qty: usize = list2.iter().filter(|&n| *n == val).count();
        total += val * i32::try_from(qty).unwrap();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
