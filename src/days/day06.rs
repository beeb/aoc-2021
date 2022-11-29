use std::collections::HashMap;

use nom::{
    character::complete::{char, u8},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day06;

fn run(input: &<Day06 as Day>::Input, days: usize) -> usize {
    // we keep a histogram of how many fishes have each timer value between 0 and 8
    let mut histogram: HashMap<u8, usize> = (0..=8).map(|x| (x, 0)).collect();
    for fish in input {
        histogram.entry(*fish).and_modify(|e| *e += 1);
    }
    for _ in 0..days {
        // first, check how many fishes are at timer 0, which means we need to create new offsprings
        let ready = histogram[&0];
        // now, decrease the count (move them) for each fish
        for timer in 1..=8 {
            histogram.insert(timer - 1, histogram[&timer]);
        }
        // now we need to move the fishes that reached zero back to 6 (there can be existing 6's)
        histogram.entry(6).and_modify(|e| *e += ready);
        // and create the new offsprings (there are no 8's left)
        histogram.insert(8, ready);
    }
    histogram.values().sum()
}

impl Day for Day06 {
    type Input = Vec<u8>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(char(','), u8)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // naive solution, keeping track of all the fishes individually
        let mut fish: Vec<u8> = input.clone();
        for _ in 1..=80 {
            for i in 0..fish.len() {
                fish[i] = match fish[i] {
                    0 => {
                        fish.push(8);
                        6
                    }
                    timer => timer - 1,
                }
            }
        }
        println!("{}", fish.len());
        // smart solution
        run(input, 80)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        run(input, 256)
    }
}
