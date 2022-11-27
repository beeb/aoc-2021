use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator,
    multi::separated_list0,
    sequence::pair,
    IResult,
};

use crate::days::Day;

pub struct Day02;

pub enum Move {
    Forward(u8),
    Down(u8),
    Up(u8),
}

impl Day for Day02 {
    type Input = Vec<Move>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            newline,
            alt((
                combinator::map(pair(tag("forward "), u8), |(_, x)| Move::Forward(x)),
                combinator::map(pair(tag("down "), u8), |(_, x)| Move::Down(x)),
                combinator::map(pair(tag("up "), u8), |(_, x)| Move::Up(x)),
            )),
        )(input)
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut depth: isize = 0;
        let mut position: isize = 0;
        for mov in input {
            match mov {
                Move::Forward(x) => position += *x as isize,
                Move::Down(x) => depth += *x as isize,
                Move::Up(x) => depth -= *x as isize,
            };
        }
        println!("depth: {depth}");
        println!("position: {position}");
        depth * position
    }

    type Output2 = isize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut aim: isize = 0;
        let mut depth: isize = 0;
        let mut position: isize = 0;
        for mov in input {
            match mov {
                Move::Forward(x) => {
                    position += *x as isize;
                    depth += (*x as isize) * aim;
                }
                Move::Down(x) => aim += *x as isize,
                Move::Up(x) => aim -= *x as isize,
            };
        }
        println!("depth: {depth}");
        println!("position: {position}");
        depth * position
    }
}
