use nom::{
    character::complete::{char, u8},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day07;

impl Day for Day07 {
    type Input = Vec<u8>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(char(','), u8)(input)
    }

    type Output1 = u8;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        unimplemented!("part_1")
    }

    type Output2 = u8;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
