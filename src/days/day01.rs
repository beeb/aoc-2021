use itertools::enumerate;
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
        let mut previous = &input[0];
        for val in input.split_at(1).1 {
            if val > previous {
                count += 1;
            }
            previous = val;
        }
        count
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut count: usize = 0;
        let mut previous_sum = 0usize;
        let mut first_meas = &input[0];
        let mut second_meas = &input[1];
        for (i, val) in enumerate(input) {
            match i {
                0 => {}
                1 => {}
                2 => {
                    previous_sum = (first_meas + second_meas + val) as usize;
                    first_meas = second_meas;
                    second_meas = val;
                }
                _ => {
                    let sum = (first_meas + second_meas + val) as usize;
                    if previous_sum < sum {
                        count += 1;
                    }
                    first_meas = second_meas;
                    second_meas = val;
                    previous_sum = sum;
                }
            };
        }
        count
    }
}
