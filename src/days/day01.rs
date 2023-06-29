use nom::{
    character::complete::{newline, u32},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<u32>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(newline, u32)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut count: usize = 0;
        for i in 1..input.len() {
            if input[i] > input[i - 1] {
                count += 1;
            }
        }
        count
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut count: usize = 0;
        let mut sums = vec![];
        for i in 1..input.len() - 1 {
            let sum = input[i - 1] + input[i] + input[i + 1];
            if !sums.is_empty() && sum > sums[sums.len() - 1] {
                count += 1;
            }
            sums.push(sum);
        }
        count
    }
}
