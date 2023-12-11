use itertools::Itertools;

pub fn day6() {
    let input = include_str!("../input/day6.txt");
    println!("day6 sum is {}", solve1(input));
    println!("day6 sum2 is {}", solve2(input));
}

fn solve1(lines: &str) -> u32 {
    let numbers = lines
        .lines()
        .map(|s| {
            s.split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect_vec()
        })
        .collect_vec();

    std::iter::zip(numbers[0].clone(), numbers[1].clone())
        .map(|(a, sum)| (0..a).filter(|i| i * (a - i) > sum).count() as u32)
        .product()
}

fn solve2(lines: &str) -> u32 {
    let numbers = lines
        .lines()
        .map(|s| {
            s.split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .map(|n| n.to_string())
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_vec();
    let n1 = numbers[0];
    let sum1 = numbers[1];

    let count = (0..n1).filter(|i| i * (n1 - i) > sum1).count() as u32;
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day6_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve1(input), 288);
        assert_eq!(solve2(input), 71503);
    }
}
