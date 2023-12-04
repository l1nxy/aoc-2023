use crate::utils;
pub fn day1() {
    let contents = utils::read_lines("./input/day1.txt");
    println!("day1 result is:{}", calc_num(contents));
}

fn calc_num(lines: Vec<String>) -> i32 {
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
        let result = calc_num(input);
        assert_eq!(142, result);
    }
}
