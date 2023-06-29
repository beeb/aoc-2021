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

fn vec_to_uint(bits: &[Vec<bool>]) -> usize {
    bits.iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (i, b)| match b {
            true => acc | 1 << (BITS - 1 - i),
            false => acc,
        })
}

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
            let zeroes = input.len() - ones;
            if ones >= zeroes {
                gamma |= 1 << (BITS - 1 - i);
            } else {
                epsilon |= 1 << (BITS - 1 - i);
            }
        }
        gamma * epsilon
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut res_oxy = input.clone();
        let mut res_co2 = input.clone();
        for i in 0..BITS {
            let items_left = res_oxy.len();
            // get all the bits at position i
            let pos: Vec<_> = res_oxy.iter().map(|entry| entry.get(i).unwrap()).collect();
            // find the number of ones
            let ones: usize = pos.iter().fold(0, |acc, x| acc + **x as usize);
            let zeroes = items_left - ones;
            res_oxy.retain(|entry| entry[i] == (ones >= zeroes));
            if res_oxy.len() <= 1 {
                break;
            }
        }
        for i in 0..BITS {
            let items_left = res_co2.len();
            // get all the bits at position i
            let pos: Vec<_> = res_co2.iter().map(|entry| entry.get(i).unwrap()).collect();
            // find the number of ones
            let ones: usize = pos.iter().fold(0, |acc, x| acc + **x as usize);
            let zeroes = items_left - ones;
            res_co2.retain(|entry| entry[i] == (ones < zeroes));
            if res_co2.len() <= 1 {
                break;
            }
        }
        let oxy = vec_to_uint(&res_oxy);
        let co2 = vec_to_uint(&res_co2);
        oxy * co2
    }
}
