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
        println!("{:?}", input);
        0
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
