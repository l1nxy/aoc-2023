use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub fn day7() {
    let input = include_str!("../input/day7.txt");
    println!("day7 sum is {}", solve1(input));
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

    //high card
    if char_map.keys().count() == 5 {
        1
    } else if char_map.keys().count() == 4 {
        // one pair
        2
    } else if char_map.keys().count() == 3 {
        // two pair or three of kind
        if char_map.values().any(|&c| c == 3) {
            // two three of kindd
            4
        } else {
            3
        }
    } else if char_map.keys().count() == 2 {
        // full house or four of a kind
        if char_map.values().any(|&c| c == 2) {
            6
        } else {
            5
        }
    } else {
        7
    }
}

fn solve1(lines: &str) -> u32 {
    // let chars_order = HashMap::from(
    //     ('A',13), ('K',12), ('Q',11), ('J',10), ('T',9), ('9',8), ('8',7), ('7',6), ('6', '5', '4', '3', '2',
    // )
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
                    let o1 = chars_order.get(&a).unwrap();
                    let o2 = chars_order.get(&b).unwrap();
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
    eprintln!("v:{:?}", v);
    v.into_iter()
        .enumerate()
        .fold(0, |sum, (i, (c, v))| sum + v * (i + 1) as u32)
}

fn solve2(lines: &str) -> u32 {
    todo!()
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
    }
}
