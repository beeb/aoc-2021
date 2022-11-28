use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u16},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult, ToUsize,
};

use crate::days::Day;

const GRID_SIZE: usize = 1024;

pub struct Day05;

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical_horizontal(&self) -> bool {
        return self.start.x == self.end.x || self.start.y == self.end.y;
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(tuple((u16, char(','), u16)), |(x, _, y)| Point {
        x: x.to_usize(),
        y: y.to_usize(),
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((parse_point, tag(" -> "), parse_point)),
        |(start, _, end)| Line { start, end },
    )(input)
}

impl Day for Day05 {
    type Input = Vec<Line>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(newline, parse_line)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let lines: Vec<_> = input
            .iter()
            .filter(|l| l.is_vertical_horizontal())
            .collect();
        let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];
        /*
        957,596 -> 957,182
        763,144 -> 69,144
        */
        let mut start_x: usize;
        let mut end_x: usize;
        let mut start_y: usize;
        let mut end_y: usize;
        for line in lines {
            if line.start.x > line.end.x {
                start_x = line.end.x;
                end_x = line.start.x;
            } else {
                start_x = line.start.x;
                end_x = line.end.x;
            }
            for i in start_x..=end_x {
                if line.start.y > line.end.y {
                    start_y = line.end.y;
                    end_y = line.start.y;
                } else {
                    start_y = line.start.y;
                    end_y = line.end.y;
                }
                for j in start_y..=end_y {
                    grid[i][j] += 1;
                }
            }
        }
        let overlaps: Vec<_> = grid.iter().flatten().filter(|x| **x > 1).collect();
        overlaps.len()
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
