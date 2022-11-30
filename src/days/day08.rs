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
        separated_list0(
            space1,
            map(alpha1, |s: &str| s.chars().sorted().collect::<String>()),
        ),
        tag(" | "),
        separated_list0(
            space1,
            map(alpha1, |s: &str| s.chars().sorted().collect::<String>()),
        ),
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

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        /*
        Segments appearance:
        a -> 0,    2, 3,    5, 6, 7, 8, 9
        b -> 0,          4, 5, 6,    8, 9
        c -> 0, 1, 2, 3, 4,       7, 8, 9
        d ->       2, 3, 4, 5, 6,    8, 9
        e -> 0,    2,          6,    8
        f -> 0, 1,    3, 4, 5, 6, 7, 8, 9
        g -> 0,    2, 3,    5, 6,    8, 9

        a = 1011011111
        b = 1000111011
        c = 1111100111
        d = 0011111011
        e = 1010001010
        f = 1101111111
        g = 1011011011
        */
        let fingerprints: HashMap<u16, char> = [
            (0b1011011111u16, 'a'),
            (0b1000111011u16, 'b'),
            (0b1111100111u16, 'c'),
            (0b0011111011u16, 'd'),
            (0b1010001010u16, 'e'),
            (0b1101111111u16, 'f'),
            (0b1011011011u16, 'g'),
        ]
        .iter()
        .cloned()
        .collect();
        0
    }
}
