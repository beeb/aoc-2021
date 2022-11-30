use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day08;

#[derive(Debug)]
pub struct Item {
    pub observations: Vec<String>,
    pub digits: Vec<String>,
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    let (rest, (observations, _, digits)) = tuple((
        separated_list0(space1, map(alpha1, |s: &str| s.to_string())),
        tag(" | "),
        separated_list0(space1, map(alpha1, |s: &str| s.to_string())),
    ))(input)?;
    Ok((
        rest,
        Item {
            observations,
            digits,
        },
    ))
}

impl Day for Day08 {
    type Input = Vec<Item>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(newline, parse_line)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let count: usize = input.iter().fold(0, |acc, i| {
            let mut count: usize = 0;
            for d in &i.digits {
                let len = d.len();
                if len == 2 || len == 3 || len == 4 || len == 7 {
                    count += 1;
                }
            }
            acc + count
        });
        count
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        /*
        Segments appearance:
        a -> 0,    2, 3,    5, 6, 7, 8, 9 = 8x
        b -> 0,          4, 5, 6,    8, 9 = 6x
        c -> 0, 1, 2, 3, 4,       7, 8, 9 = 8x
        d ->       2, 3, 4, 5, 6,    8, 9 = 7x
        e -> 0,    2,          6,    8    = 4x
        f -> 0, 1,    3, 4, 5, 6, 7, 8, 9 = 9x
        g -> 0,    2, 3,    5, 6,    8, 9 = 7x
        */
        let segments_to_digit: HashMap<String, usize> = [
            ("abcefg".to_string(), 0),
            ("cf".to_string(), 1),
            ("acdeg".to_string(), 2),
            ("acdfg".to_string(), 3),
            ("bcdf".to_string(), 4),
            ("abdfg".to_string(), 5),
            ("abdefg".to_string(), 6),
            ("acf".to_string(), 7),
            ("abcdefg".to_string(), 8),
            ("abcdfg".to_string(), 9),
        ]
        .into_iter()
        .collect();
        let mut total: usize = 0;
        for item in input {
            let all_chars = item.observations.join("");
            let counts = all_chars.chars().counts();
            let mut corresp: HashMap<char, char> = HashMap::new();
            let mut dg: Vec<char> = vec![];
            let mut ac: Vec<char> = vec![];
            for wrong_char in 'a'..='g' {
                match counts[&wrong_char] {
                    4 => {
                        corresp.insert(wrong_char, 'e');
                    }
                    6 => {
                        corresp.insert(wrong_char, 'b');
                    }
                    9 => {
                        corresp.insert(wrong_char, 'f');
                    }
                    7 => {
                        // can be either 'd' or 'g'
                        dg.push(wrong_char);
                    }
                    8 => {
                        // can be either 'a' or 'c'
                        ac.push(wrong_char);
                    }
                    _ => {}
                }
            }
            // from part 1: 'g' only appears in digit 8, so it's present in digit 4, it's 'd'
            // from part 1: 'c' appears in all 4 unique digits, so if count is 4 then it's 'c'
            let count_dg: u8 = item
                .observations
                .iter()
                .filter(|obs| obs.len() == 4)
                .fold(0, |acc, obs| acc + obs.contains(dg[0]) as u8);
            let count_ac: u8 = item
                .observations
                .iter()
                .filter(|obs| obs.len() == 2 || obs.len() == 3 || obs.len() == 4 || obs.len() == 7)
                .fold(0, |acc, obs| acc + obs.contains(ac[0]) as u8);

            if count_dg == 1 {
                corresp.insert(dg[0], 'd');
                corresp.insert(dg[1], 'g');
            } else {
                corresp.insert(dg[1], 'd');
                corresp.insert(dg[0], 'g');
            }

            if count_ac == 4 {
                corresp.insert(ac[0], 'c');
                corresp.insert(ac[1], 'a');
            } else {
                corresp.insert(ac[1], 'c');
                corresp.insert(ac[0], 'a');
            }

            // we now have the correspondance
            let mut value: usize = 0;
            for (i, d) in item.digits.iter().enumerate() {
                let decoded = d.chars().map(|c| corresp[&c]).sorted().collect::<String>();
                let digit = segments_to_digit.get(&decoded).expect("has to be");
                value += digit * 10usize.pow(3 - i as u32);
            }
            total += value;
        }
        total
    }
}
