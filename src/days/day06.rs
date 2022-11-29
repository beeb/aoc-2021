use nom::{
    character::complete::{char, u64},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day06;

impl Day for Day06 {
    type Input = Vec<u64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(char(','), u64)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut fish = input.clone();
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
        let mut counter: u64 = 0;
        for fish in input {
            counter += total_count(*fish, 80);
        }
        counter
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        // for each fish, how many offsprings will it have after x days?
        // it starts making offsprings after timer days
        // so it produces offsprings for (x - timer) days
        // so for a given fish, ceil((x - timer) / 7) is the number of offsprings
        // and this is recursive, so for each of those offsprings, they also will generate ceil((y - timer) / 7)
        // offsprings, where y is the remaining days (x - initial_parent_timer - 1) when they were spawned and
        // timer is 8 for them
        let mut counter: u64 = 0;
        for fish in input {
            counter += total_count(*fish, 256);
        }
        counter
    }
}

fn total_count(fish_counter: u64, days_left: u64) -> u64 {
    let mut total: u64 = 1;
    let num_descendants = ((days_left - fish_counter) + 7 - 1) / 7; // ceil division
    for _ in 0..num_descendants {
        total += total_count(7, days_left - fish_counter - 1);
    }
    total
}
