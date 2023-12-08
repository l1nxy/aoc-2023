use itertools::Itertools;

pub fn day5() {
    let lines = include_str!("../input/day5.txt");
    let sum = solve1(lines);
    println!("ths day5 sum is {}", sum);
}

struct Map {
    list: Vec<(u32, u32, u32)>,
}

fn solve1(lines: &str) -> u32 {
    let mut lines_iter = lines.lines();
    let seed = lines_iter.next().unwrap();
    let seed_list = seed
        .splitn(2, ':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    //splist by empty line
    let l = lines.split("\n\n").collect_vec();
    println!("l {:?}", l);

    todo!();
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_day5() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(solve1(input), 35);
    }
}
