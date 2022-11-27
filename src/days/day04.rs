use nom::{
    character::complete::{char, newline, space0, u8},
    combinator::{self, opt},
    multi::{count, many1, separated_list0},
    sequence::pair,
    IResult,
};

use crate::days::Day;

pub struct Day04;

fn parse_row(input: &str) -> IResult<&str, [u8; 5]> {
    let (rest, row) = count(combinator::map(pair(space0, u8), |x| x.1), 5)(input)?;
    Ok((rest, row.try_into().unwrap()))
}

fn parse_grid(input: &str) -> IResult<&str, [[u8; 5]; 5]> {
    let (rest, grid) = count(combinator::map(pair(parse_row, opt(newline)), |x| x.0), 5)(input)?;
    Ok((rest, grid.try_into().unwrap()))
}

impl Day for Day04 {
    type Input = (Vec<u8>, Vec<[[u8; 5]; 5]>);

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, (draw, _)) = pair(separated_list0(char(','), u8), count(newline, 2))(input)?;
        let (rest, boards) = separated_list0(many1(newline), parse_grid)(rest)?;
        Ok((rest, (draw.try_into().unwrap(), boards.try_into().unwrap())))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (draw, boards) = input;
        let mut winner: Option<usize> = None;
        let mut final_drawn: Option<Vec<u8>> = None;
        for i in 1..=draw.len() {
            for j in 0..boards.len() {
                let (drawn, _) = draw.split_at(i);
                for row in boards[j] {
                    if row.iter().all(|x| drawn.contains(x)) {
                        winner = Some(j);
                        final_drawn = Some(drawn.to_vec());
                        break;
                    }
                }
                for k in 0..5 {
                    let col: Vec<u8> = boards[j].iter().map(|x| x[k]).collect();
                    if col.iter().all(|x| drawn.contains(x)) {
                        winner = Some(j);
                        final_drawn = Some(drawn.to_vec());
                        break;
                    }
                }
                if winner.is_some() {
                    break;
                }
            }
            if winner.is_some() {
                break;
            }
        }
        if winner.is_none() {
            panic!("Could not find a winning board");
        }
        let win = boards[winner.unwrap()];
        let drawn = final_drawn.unwrap();
        println!("{:?}", win);
        println!("{:?}", drawn);
        let sum = win
            .to_vec()
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>()
            .iter()
            .flatten()
            .fold(0usize, |acc, x| match drawn.contains(x) {
                true => acc,
                false => acc + *x as usize,
            });
        println!("{sum}");
        sum * (*drawn.last().unwrap() as usize)
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
