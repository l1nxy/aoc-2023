use crate::utils;
pub fn day1() {
    let contents = utils::read_lines("./input/day1_2.txt");
    println!("day1 result is:{}", part1(contents.clone()));
    println!("day1_2 result is:{}", part2(contents));
}

fn part1(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|s| {
            let numeric_chars: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
            let first = numeric_chars.first().unwrap();
            let last = numeric_chars.last().unwrap();
            let sn = format!("{}{}", first, last);
            sn.parse::<i32>().unwrap_or_default()
        })
        .sum()
}

const NUMBERS: &'static [&str] = &[
    "zero", "0", "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
    "seven", "7", "eight", "8", "nine", "9",
];
fn part2(lines: Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| {
            let mut numbers = NUMBERS
                .iter()
                .enumerate()
                .map(|(i, s)| line.match_indices(s).map(move |index| (index, i / 2)))
                .flatten()
                .collect::<Vec<_>>();

            numbers.sort_by(|a, b| a.0.cmp(&b.0));
            numbers
                .first()
                .map(|f| {
                    format!("{}{}", f.1, numbers.last().unwrap().1)
                        .parse::<u64>()
                        .unwrap()
                })
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = vec![
            "1abc2".into(),
            "pqr3stu8vwx".into(),
            "a1b2c3d4e5f".into(),
            "treb7uchet".into(),
        ];

        let input2 = vec![
            "two1nine".into(),
            "eightwothree".into(),
            "abcone2threexyz".into(),
            "xtwone3four".into(),
            "4nineeightseven2".into(),
            "zoneight234".into(),
            "7pqrstsixteen".into(),
        ];

        assert_eq!(part2(input2), 281);
        let result = part1(input);
        assert_eq!(142, result);
    }
}
