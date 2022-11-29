use nom::{
    character::complete::{char, u64},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day07;

fn fuel_spent_part1(input: &<Day07 as Day>::Input, pos: u64) -> u64 {
    input
        .iter()
        .map(|i| {
            if i < &pos {
                return pos - i;
            }
            i - pos
        })
        .sum()
}

impl Day for Day07 {
    type Input = Vec<u64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(char(','), u64)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // the position the requires the least horizontal movement on average is the median of the list
        // to calculate the median of a discrete distribution, one can take the value in the middle of the sorted list
        let mut input = input.clone();
        input.sort();
        // if the number of items is pair, we have to average the value on either side of the middle
        let pos = match input.len() % 2 {
            0 => {
                let left = input[(input.len() / 2) - 1];
                let right = input[input.len() / 2];
                (left + right) / 2
            }
            _ => input[(input.len() / 2)],
        };
        // get the fuel spent to go to that position for each crab submarine
        fuel_spent_part1(&input, pos)
    }

    type Output2 = u64;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
