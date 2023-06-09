use std::str::FromStr;

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err("Invalid move".to_string()),
        }
    }
}

pub fn process_part_1(input: &str) -> String {
    let score: i32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            let opponent = moves[0].parse::<Move>().unwrap();

            match moves[1] {
                "X" => {
                    1 + (match opponent {
                        Move::Rock => 3,
                        Move::Paper => 0,
                        Move::Scissors => 6,
                    })
                }
                "Y" => {
                    2 + (match opponent {
                        Move::Rock => 6,
                        Move::Paper => 3,
                        Move::Scissors => 0,
                    })
                }
                "Z" => {
                    3 + (match opponent {
                        Move::Rock => 0,
                        Move::Paper => 6,
                        Move::Scissors => 3,
                    })
                }
                _ => panic!("Invalid move"),
            }
        })
        .sum();

    score.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let score: i32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            let opponent = moves[0].parse::<Move>().unwrap();

            match moves[1] {
                "X" => {
                    (match opponent {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    }) as i32
                }
                "Y" => 3 + opponent as i32,
                "Z" => {
                    6 + (match opponent {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    }) as i32
                }
                _ => panic!("Invalid move"),
            }
        })
        .sum();

    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_process_part_1() {
        assert_eq!("15", process_part_1(INPUT));
    }

    #[test]
    fn test_process_part_2() {
        assert_eq!("12", process_part_2(INPUT));
    }
}
