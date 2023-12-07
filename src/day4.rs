use itertools::Itertools;

pub fn day4() {
    let input = include_str!("../input/day4.txt");
    let sum = solve(input);
    let sum2 = solve2(input);
    println!("day4 sum is: {}", sum);
    println!("day4 part2 sum is: {}", sum2);
}

fn solve2(lines: &str) -> u32 {
    let count = lines.lines().count();
    let mut flags = vec![1; count];

    lines.lines().into_iter().enumerate().for_each(|(i, s)| {
        let x: Vec<Vec<i32>> = s
            .splitn(2, ':')
            .last()
            .unwrap()
            .split('|')
            .map(|ss| {
                ss.trim()
                    .split(' ')
                    .filter_map(|n| {
                        if n.len() == 0 {
                            None
                        } else {
                            Some(n.parse::<i32>().unwrap())
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        let count = x[0]
            .iter()
            .cartesian_product(x[1].iter())
            .filter_map(|(l, r)| if l == r { Some(*l) } else { None })
            .count();

        (0..flags[i]).into_iter().for_each(|_| {
            (i + 1..i + count + 1).into_iter().for_each(|ii| {
                flags[ii] += 1;
            });
        });
    });

    flags.iter().sum()
}

fn solve(lines: &str) -> u32 {
    lines
        .lines()
        .into_iter()
        .map(|s| {
            let x = s
                .splitn(2, ':')
                .last()
                .unwrap()
                .split('|')
                .map(|ss| {
                    ss.trim()
                        .split(' ')
                        .filter_map(|n| {
                            if n.len() == 0 {
                                None
                            } else {
                                Some(n.parse::<i32>().unwrap())
                            }
                        })
                        .collect_vec()
                })
                .collect_vec();

            let count = x[0]
                .iter()
                .cartesian_product(x[1].iter())
                .filter_map(|(l, r)| if l == r { Some(*l) } else { None })
                .count();

            if count == 0 {
                0
            } else {
                2u32.pow((count - 1) as u32)
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_day4_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let sum = solve(input);
        let sum2 = solve2(input);
        assert_eq!(13, sum);
        assert_eq!(30, sum2);
    }
}
