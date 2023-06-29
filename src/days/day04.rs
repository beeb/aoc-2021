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

/// calculate the sum of all numbers on the board which were not drawn and multiply with last drawn number
fn calculate_score(board: [[u8; 5]; 5], drawn: Vec<u8>) -> usize {
    let sum = board
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
    sum * (*drawn.last().unwrap() as usize)
}

impl Day for Day04 {
    type Input = (Vec<u8>, Vec<[[u8; 5]; 5]>);

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, (draw, _)) = pair(separated_list0(char(','), u8), count(newline, 2))(input)?;
        let (rest, boards) = separated_list0(many1(newline), parse_grid)(rest)?;
        Ok((rest, (draw, boards)))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (draw, boards) = input;
        let mut winner: Option<usize> = None; // which board is the first winner
        let mut winning_round: Option<usize> = None; // what was the winning round

        // draw each number
        for i in 1..=draw.len() {
            // check if board if we have a win
            for (j, board) in boards.iter().enumerate() {
                let (drawn, _) = draw.split_at(i);
                // check rows first
                for row in board {
                    if row.iter().all(|x| drawn.contains(x)) {
                        winner = Some(j);
                        winning_round = Some(i);
                        break;
                    }
                }
                // then check cols
                for k in 0..5 {
                    let col: Vec<u8> = board.iter().map(|x| x[k]).collect();
                    if col.iter().all(|x| drawn.contains(x)) {
                        winner = Some(j);
                        winning_round = Some(i);
                        break;
                    }
                }
                // if we have a winner we can break out of the loop early
                if winner.is_some() {
                    break;
                }
            }
            // break early if we have found the winner
            if winner.is_some() {
                break;
            }
        }
        assert!(winner.is_some());
        let win = boards[winner.unwrap()]; // board that wins first
        let (drawn, _) = draw.split_at(winning_round.unwrap()); // which numbers were drawn
        println!("{:?}", win);
        println!("{:?}", drawn);
        calculate_score(win, drawn.to_vec())
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (draw, boards) = input;
        let mut winning_rounds: Vec<usize> = vec![]; // for each board, which is the winning round
        for (j, board) in boards.iter().enumerate() {
            for i in 1..=draw.len() {
                let (drawn, _) = draw.split_at(i);
                // check rows first
                for row in board {
                    if row.iter().all(|x| drawn.contains(x)) {
                        // win
                        winning_rounds.push(i);
                        break;
                    }
                }
                // then check columns
                for k in 0..5 {
                    let col: Vec<u8> = board.iter().map(|x| x[k]).collect();
                    if col.iter().all(|x| drawn.contains(x)) {
                        // win
                        winning_rounds.push(i);
                        break;
                    }
                }
                if winning_rounds.len() > j {
                    // if we found the winning round for this board, we can break to proceed to the next board
                    break;
                }
            }
            if winning_rounds.len() <= j {
                // in case there was no winning round, save a sentinel value
                winning_rounds.push(999);
            }
        }
        assert!(winning_rounds.len() == boards.len());
        // get the position of the maximum value, so which board wins last
        let (argmax, winning_round) = winning_rounds
            .iter()
            .enumerate()
            .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
            .unwrap();
        let win_last = boards[argmax]; // this board wins last
        let (drawn, _) = draw.split_at(*winning_round);
        calculate_score(win_last, drawn.to_vec())
    }
}
