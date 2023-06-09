use std::collections::HashMap;

pub fn process_part_1(input: &str) -> String {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, letter)| (letter, index + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let count = line.chars().count();
            let part1 = &line[0..count / 2];
            let part2 = &line[count / 2..count];

            let found = part1.chars().find(|c| part2.contains(*c)).unwrap();
            letters.get(&found).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let letters = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, letter)| (letter, index + 1))
        .collect::<HashMap<char, usize>>();

    let v: Vec<_> = input.lines().collect();
    let result: usize = v
        .chunks(3)
        .map(|loads| {
            let found = loads[0]
                .chars()
                .find(|letter| loads[1].contains(*letter) && loads[2].contains(*letter))
                .unwrap();

            letters.get(&found).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_process_part_1() {
        assert_eq!("157", process_part_1(INPUT));
    }

    #[test]
    fn test_process_part_2() {
        assert_eq!("70", process_part_2(INPUT));
    }
}
