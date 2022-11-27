use itertools::enumerate;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator,
    multi::{count, separated_list0},
    IResult,
};

use crate::days::Day;

pub struct Day03;

const BITS: usize = 12;

impl Day for Day03 {
    type Input = Vec<Vec<bool>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            newline,
            count(
                combinator::map(alt((char('0'), char('1'))), |x| x != '0'),
                BITS,
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let majority = input.len() / 2;
        let mut input_iters: Vec<_> = input.iter().map(|entry| entry.iter()).collect();
        let transpose: Vec<Vec<_>> = (0..input[0].len())
            .map(|_| {
                input_iters
                    .iter_mut()
                    .map(|entry| entry.next().unwrap())
                    .collect()
            })
            .collect();
        assert!(transpose.len() == BITS);
        let mut gamma: usize = 0;
        let mut epsilon: usize = 0;
        for (i, pos) in enumerate(transpose) {
            let ones: usize = pos.iter().fold(0, |acc, x| acc + **x as usize);
            if ones >= majority {
                gamma = gamma | 1 << BITS - 1 - i;
            } else {
                epsilon = epsilon | 1 << BITS - 1 - i;
            }
        }
        println!("{gamma:12b}\n{epsilon:12b}");
        gamma * epsilon
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
