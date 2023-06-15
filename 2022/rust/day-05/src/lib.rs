use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_cell(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((tag("   "), delimited(tag("["), complete::alpha1, tag("]"))))(input)?;

    Ok((
        input,
        match c {
            "   " => None,
            _ => Some(c),
        },
    ))
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, res) = separated_list1(tag(" "), parse_cell)(input)?;
    Ok((input, res))
}

pub fn parse_block(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    let (input, res) = separated_list1(newline, parse_line)(input)?;
    Ok((input, res))
}

#[derive(Debug)]
pub struct Move {
    pub quantity: u32,
    pub from: u32,
    pub to: u32,
}

pub fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, quantity) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            quantity,
            from: from - 1,
            to: to - 1,
        },
    ))
}

pub fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, res) = separated_list1(newline, parse_move)(input)?;
    Ok((input, res))
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn process_part_1(input: &str) -> String {
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let (_, tree_init) = parse_block(data[0]).unwrap();
    let (_, moves_rules) = parse_moves(data[1]).unwrap();

    let raw_stacks: Vec<Vec<Option<&str>>> = transpose(tree_init);
    let mut stacks: Vec<Vec<&str>> = vec![];

    for stack in raw_stacks {
        let mut new_col: Vec<&str> = vec![];
        for item in stack {
            if item.is_none() {
                continue;
            }
            new_col.push(item.unwrap());
        }
        stacks.push(new_col);
    }

    for move_rule in moves_rules {
        let blocks = stacks[move_rule.from as usize]
            .drain(0..move_rule.quantity as usize)
            .collect::<Vec<&str>>();
        for block in blocks {
            stacks[move_rule.to as usize].insert(0, block);
        }
    }

    stacks
        .iter()
        .map(|n| n.first().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn process_part_2(input: &str) -> String {
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let (_, tree_init) = parse_block(data[0]).unwrap();
    let (_, moves_rules) = parse_moves(data[1]).unwrap();

    let raw_stacks: Vec<Vec<Option<&str>>> = transpose(tree_init);
    let mut stacks: Vec<Vec<&str>> = vec![];

    for stack in raw_stacks {
        let mut new_col: Vec<&str> = vec![];
        for item in stack {
            if item.is_none() {
                continue;
            }
            new_col.push(item.unwrap());
        }
        stacks.push(new_col);
    }

    for move_rule in moves_rules {
        let mut blocks = stacks[move_rule.from as usize]
            .drain(0..move_rule.quantity as usize)
            .collect::<Vec<&str>>();

        blocks.reverse();

        for block in blocks {
            stacks[move_rule.to as usize].insert(0, block);
        }
    }

    stacks
        .iter()
        .map(|n| n.first().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_process_part_1() {
        assert_eq!("CMZ", process_part_1(INPUT));
    }

    #[test]
    fn test_process_part_2() {
        assert_eq!("MCD", process_part_2(INPUT));
    }
}
