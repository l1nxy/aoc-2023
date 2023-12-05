use lazy_static::lazy_static;
#[allow(dead_code)]
use nom::{
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{tuple, Tuple},
    AsChar, IResult,
};
use std::collections::HashMap;

lazy_static! {
    static ref LIMITS: HashMap<String, u32> = {
        let mut m = HashMap::new();
        m.insert("red".to_string(), 12);
        m.insert("green".to_string(), 13);
        m.insert("blue".to_string(), 14);
        m
    };
}

pub fn day2_1() {
    let lines = include_str!("../input/day2.txt");
    println!("sum is {}", solve(lines));
}

fn solve(lines: &str) -> u32 {
    lines
        .lines()
        .into_iter()
        .filter_map(|s| {
            let (input, game) = parse_by_nom(s).unwrap();
            eprintln!("{input}");

            if game
                .ball_list
                .iter()
                .flatten()
                .any(|(count, color)| count > LIMITS.get(color).unwrap_or(&0))
            {
                return None;
            }

            Some(game.id)
        })
        .sum()
}

struct Game {
    id: u32,
    ball_list: Vec<Vec<(u32, String)>>,
}

fn parse_count_color(input: &str) -> IResult<&str, (u32, String)> {
    let color = take_while1(|c: char| c.is_alpha());
    let number_parser = map_res(digit1, str::parse::<u32>);
    let (input, (_, count, _, color)) = tuple((tag(" "), number_parser, tag(" "), color))(input)?;
    Ok((input, (count, color.to_string())))
}

fn parse_by_nom(input: &str) -> IResult<&str, Game> {
    let game_id = map_res(take_while_m_n(1, 5, |c: char| c.is_numeric()), |s: &str| {
        s.parse::<u32>()
    });

    let split_list = separated_list1(tag(","), parse_count_color);
    let split_list2 = separated_list1(tag(";"), split_list);

    let (input, (_, _, id, _, ball_list)) =
        (tag("Game"), tag(" "), game_id, tag(":"), split_list2).parse(input)?;

    println!("game id is:{}, ball_list : {:?}", id, ball_list);

    Ok((input, Game { id, ball_list }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_1() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let sum = solve(input.join("\n").as_str());
        assert_eq!(sum, 8);
    }
}
