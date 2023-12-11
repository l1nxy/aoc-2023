use itertools::Itertools;
use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap};

lazy_static! {
    static ref CHARS_ORDER: HashMap<char, u32> = {
        let mut chars_order = HashMap::new();
        chars_order.insert('A', 13);
        chars_order.insert('K', 12);
        chars_order.insert('Q', 11);
        chars_order.insert('J', 10);
        chars_order.insert('T', 9);
        chars_order.insert('9', 8);
        chars_order.insert('8', 7);
        chars_order.insert('7', 6);
        chars_order.insert('6', 5);
        chars_order.insert('5', 4);
        chars_order.insert('4', 3);
        chars_order.insert('3', 2);
        chars_order.insert('2', 1);
        chars_order
    };
    static ref CHARS_ORDER2: HashMap<char, u32> = {
        let mut chars_order = HashMap::new();
        chars_order.insert('A', 13);
        chars_order.insert('K', 12);
        chars_order.insert('Q', 11);
        chars_order.insert('T', 10);
        chars_order.insert('9', 9);
        chars_order.insert('8', 8);
        chars_order.insert('7', 7);
        chars_order.insert('6', 6);
        chars_order.insert('5', 5);
        chars_order.insert('4', 4);
        chars_order.insert('3', 3);
        chars_order.insert('2', 2);
        chars_order.insert('J', 1);
        chars_order
    };
}

pub fn day7() {
    let input = include_str!("../input/day7.txt");
    println!("day7 sum is {}", solve1(input));
    println!("day7 sum part2 is {}", solve2(input));
}

fn get_card_type(card: &str) -> u8 {
    let mut char_map: HashMap<char, usize> = HashMap::new();
    card.chars().for_each(|c| {
        char_map
            .entry(c)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    });

    let key_count = char_map.keys().count();

    match key_count {
        5 => 1,
        4 => 2,
        3 => {
            if char_map.values().any(|&c| c == 3) {
                4
            } else {
                3
            }
        }
        2 => {
            if char_map.values().any(|&c| c == 2) {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

fn get_card_type_part2(card: &str) -> u8 {
    let mut char_map: HashMap<char, usize> = HashMap::new();
    card.chars().for_each(|c| {
        char_map
            .entry(c)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    });

    let key_count = char_map.keys().count();

    let &j_count = char_map.get(&'J').unwrap_or(&0);

    match key_count {
        5 => match j_count {
            // up to one pair
            1 => 2,
            _ => 1,
        },
        4 => {
            match j_count {
                // up to Three of a kind
                1 | 2 => 4,
                //otherwise still one paire
                _ => 2,
            }
        }
        3 => match j_count {
            1 => {
                // has 1 or 3 j
                if char_map.values().any(|&c| c == 3) {
                    // up to Four of a kind
                    6
                } else {
                    //two pair can up to Full house
                    5
                }
            }
            2 | 3 => 6,
            _ => {
                if char_map.values().any(|&c| c == 3) {
                    4
                } else {
                    3
                }
            }
        },
        2 => match j_count {
            1..=4 => 7,
            _ => {
                if char_map.values().any(|&c| c == 2) {
                    5
                } else {
                    6
                }
            }
        },
        _ => 7,
    }
}

fn solve1(lines: &str) -> u32 {
    let v = lines
        .lines()
        .filter_map(|s| {
            s.split_once(' ')
                .map(|(card, bid)| (card, bid.parse::<u32>().unwrap()))
        })
        .sorted_by(|(card1, _), (card2, _)| {
            let type1 = get_card_type(card1);
            let type2 = get_card_type(card2);
            if type1 == type2 {
                let pairs = card1.chars().zip(card2.chars()).collect_vec();
                for (a, b) in pairs {
                    let o1 = CHARS_ORDER.get(&a).unwrap();
                    let o2 = CHARS_ORDER.get(&b).unwrap();
                    if o1 != o2 {
                        return o1.cmp(o2);
                    } else {
                        Ordering::Equal
                    };
                }

                Ordering::Equal
            } else {
                type1.cmp(&type2)
            }
        })
        .collect_vec();

    v.into_iter()
        .enumerate()
        .fold(0, |sum, (i, (_, v))| sum + v * (i + 1) as u32)
}

fn solve2(lines: &str) -> u32 {
    let v = lines
        .lines()
        .filter_map(|s| {
            s.split_once(' ')
                .map(|(card, bid)| (card, bid.parse::<u32>().unwrap()))
        })
        .sorted_by(|(card1, _), (card2, _)| {
            let type1 = get_card_type_part2(card1);
            println!("card1 {card1} type2 :{type1}");
            let type2 = get_card_type_part2(card2);
            println!("card2 {card2} type2 :{type2}");
            if type1 == type2 {
                let pairs = card1.chars().zip(card2.chars()).collect_vec();
                for (a, b) in pairs {
                    let o1 = CHARS_ORDER2.get(&a).unwrap();
                    let o2 = CHARS_ORDER2.get(&b).unwrap();
                    if o1 != o2 {
                        return o1.cmp(o2);
                    } else {
                        Ordering::Equal
                    };
                }

                Ordering::Equal
            } else {
                type1.cmp(&type2)
            }
        })
        .collect_vec();

    println!("v is {:?}", v);
    v.into_iter()
        .enumerate()
        .fold(0, |sum, (i, (_, v))| sum + v * (i + 1) as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day7_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve1(input), 6440);
        assert_eq!(solve2(input), 5905);
    }
}
