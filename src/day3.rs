use std::collections::HashSet;

use itertools::Itertools;

pub fn solve2(lines: &str) -> u32 {
    let grid: Vec<Vec<char>> = lines
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let calc_offset = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut flags: HashSet<(usize, usize)> = HashSet::new();

    let all_n: Vec<(i32, (i32, i32))> = (0..rows)
        .cartesian_product(0..cols)
        .into_iter()
        .filter_map(|(i, j)| {
            if grid[i][j].is_digit(10) && flags.get(&(i, j)).is_none() {
                let mut number_str = String::new();
                let mut coords: Vec<(usize, usize)> = Vec::new();

                number_str.push(grid[i][j]);
                coords.push((i, j));
                flags.insert((i, j));

                let mut k = j + 1;
                while k < cols && grid[i][k].is_digit(10) {
                    number_str.push(grid[i][k]);
                    flags.insert((i, k));
                    coords.push((i, k));

                    k += 1;
                }

                let mut chars: Vec<char> = Vec::new();

                let mut star_x = -1i32;
                let mut star_y = -1i32;
                coords.iter().for_each(|(a, b)| {
                    let eight_dir = calc_offset
                        .iter()
                        .map(|(x, y)| (*a as i32 + x, *b as i32 + y))
                        .collect::<Vec<(i32, i32)>>();

                    eight_dir.iter().for_each(|(x, y)| {
                        if *x >= 0 && *y >= 0 {
                            if (*x as usize) < rows && (*y as usize) < cols {
                                let c = grid[*x as usize][*y as usize];
                                if c.is_numeric() {
                                    return;
                                }
                                chars.push(c);
                                if c == '*' {
                                    star_x = *x as i32;
                                    star_y = *y as i32;
                                }
                            }
                        }
                    });
                });

                if chars.iter().any(|c| *c == '*') {
                    Some((number_str.parse::<i32>().unwrap_or(1), (star_x, star_y)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec();

    all_n
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(l, r)| l.1 .0 == r.1 .0 && l.1 .1 == r.1 .1)
        .map(|(l, r)| l.0 * r.0)
        .sum::<i32>() as u32
}

pub fn solve(lines: &str) -> u32 {
    let grid: Vec<Vec<char>> = lines
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let calc_offset = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut flags: HashSet<(usize, usize)> = HashSet::new();

    (0..rows)
        .cartesian_product(0..cols)
        .into_iter()
        .map(|(i, j)| {
            if grid[i][j].is_digit(10) && flags.get(&(i, j)).is_none() {
                let mut number_str = String::new();
                let mut coords: Vec<(usize, usize)> = Vec::new();

                number_str.push(grid[i][j]);
                coords.push((i, j));
                flags.insert((i, j));

                let mut k = j + 1;
                while k < cols && grid[i][k].is_digit(10) {
                    number_str.push(grid[i][k]);
                    flags.insert((i, k));
                    coords.push((i, k));

                    k += 1;
                }

                let mut chars: Vec<char> = Vec::new();

                coords.iter().for_each(|(a, b)| {
                    let eight_dir = calc_offset
                        .iter()
                        .map(|(x, y)| (*a as i32 + x, *b as i32 + y))
                        .collect::<Vec<(i32, i32)>>();

                    eight_dir.iter().for_each(|(x, y)| {
                        if *x >= 0 && *y >= 0 {
                            if (*x as usize) < rows && (*y as usize) < cols {
                                if grid[*x as usize][*y as usize].is_numeric() {
                                    return;
                                }
                                chars.push(grid[*x as usize][*y as usize]);
                            }
                        }
                    });
                });

                if !chars.iter().all(|c| *c == '.') {
                    number_str.parse::<i32>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<i32>() as u32
}

pub fn day3() {
    let lines = include_str!("../input/day3.txt");
    println!("sum is {}", solve(lines));
    println!("sum2 is {}", solve2(lines));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day3() {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598.. ";

        let sum = solve(input);
        let sum2 = solve2(input);
        assert_eq!(4361, sum);
        assert_eq!(467835, sum2);
    }
}
