use nom::IResult;

use crate::days::Day;

pub struct Day09;

impl Day for Day09 {
    type Input = String;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        unimplemented!("parser")
    }

    type Output1 = String;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        unimplemented!("part_1")
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
