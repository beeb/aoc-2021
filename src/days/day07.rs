use std::collections::HashMap;

use nom::{
    character::complete::{char, u64},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day07;

fn median(input: &<Day07 as Day>::Input) -> u64 {
    // the position the requires the least horizontal movement on average is the median of the list
    // to calculate the median of a discrete distribution, one can take the value in the middle of the sorted list
    let mut input = input.clone();
    input.sort();
    // if the number of items is pair, we have to average the value on either side of the middle
    match input.len() % 2 {
        0 => {
            let left = input[(input.len() / 2) - 1];
            let right = input[input.len() / 2];
            (left + right) / 2
        }
        _ => input[(input.len() / 2)],
    }
}

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

struct MemoizedFib {
    pub mem: HashMap<u64, u64>,
}

impl MemoizedFib {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    fn fuel_spent_for_distance(&mut self, dist: u64) -> u64 {
        if dist == 0 {
            return 0;
        }
        if let Some(prev) = self.mem.get(&(dist - 1)) {
            let new = prev + dist;
            self.mem.insert(dist, new);
            return new;
        }
        let new = self.fuel_spent_for_distance(dist - 1) + dist;
        self.mem.insert(dist, new);
        new
    }

    fn fuel_spent(&mut self, input: &<Day07 as Day>::Input, pos: u64) -> u64 {
        input
            .iter()
            .map(|i| {
                if i < &pos {
                    return self.fuel_spent_for_distance(pos - i);
                }
                self.fuel_spent_for_distance(i - pos)
            })
            .sum()
    }
}

impl Day for Day07 {
    type Input = Vec<u64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(char(','), u64)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let pos = median(&input);
        // get the fuel spent to go to that position for each crab submarine
        fuel_spent_part1(&input, pos)
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut fib = MemoizedFib::new();
        let mut pos = median(&input);
        let mut last = fib.fuel_spent(&input, pos);
        loop {
            let new = fib.fuel_spent(&input, pos + 1);
            if new > last {
                break;
            }
            pos += 1;
            last = new;
        }
        println!("{}", pos);
        last
    }
}
