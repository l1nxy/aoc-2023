use itertools::Itertools;
use rayon::prelude::*;

pub fn day5() {
    let lines = include_str!("../input/day5.txt");
    let sum = solve1(lines);
    println!("ths day5 sum is {}", sum);
    let sum2 = solve2(lines);
    println!("ths day5 part2 sum is {}", sum2);
}

fn solve2(lines: &str) -> u64 {
    //splist by empty line
    let line_vec = lines.split("\n\n").collect_vec();
    let seed_list = line_vec
        .first()
        .unwrap()
        .splitn(2, ':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let map_list = line_vec
        .iter()
        .skip(1)
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|ss| {
                    ss.split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    seed_list
        .par_chunks(2)
        .filter_map(|v| {
            (v[0]..(v[0] + v[1]))
                .map(|n| {
                    let ret = map_list.iter().fold(n, |acc, x| {
                        x.par_iter()
                            .find_any(|i| acc >= i[1] && acc <= i[1] + i[2])
                            .map(|xx| xx[0] + (acc - xx[1]))
                            .unwrap_or(acc)
                    });
                    (n, ret)
                })
                .min_by(|x, y| x.1.cmp(&y.1))
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0
}

fn solve1(lines: &str) -> u64 {
    //splist by empty line
    let line_vec = lines.split("\n\n").collect_vec();
    let seed_list = line_vec
        .first()
        .unwrap()
        .splitn(2, ':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let map_list = line_vec
        .iter()
        .skip(1)
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|ss| {
                    ss.split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    seed_list
        .iter()
        .map(|n| {
            map_list.iter().fold(*n, |acc, x| {
                x.iter()
                    .find(|i| acc >= i[1] && acc <= i[1] + i[2])
                    .map(|xx| xx[0] + (acc - xx[1]))
                    .unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
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

        assert_eq!(solve2(input), 82);
    }
}
