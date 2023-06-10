pub fn process_part_1(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let ranges: Vec<(u32, u32)> = line
                .split(',')
                .map(|r| {
                    r.split_once('-')
                        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                        .unwrap()
                })
                .collect();
            let a_in_b = ranges[0].0.ge(&ranges[1].0) && ranges[0].1.le(&ranges[1].1);
            let b_in_a = ranges[1].0.ge(&ranges[0].0) && ranges[1].1.le(&ranges[0].1);
            a_in_b | b_in_a
        })
        .count();
    result.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let ranges: Vec<(u32, u32)> = line
                .split(',')
                .map(|r| {
                    r.split_once('-')
                        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                        .unwrap()
                })
                .collect();

            let a_not_b = ranges[0].1.lt(&ranges[1].0);
            let b_not_a = ranges[1].1.lt(&ranges[0].0);
            !(a_not_b || b_not_a)
        })
        .count();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_process_part_1() {
        assert_eq!("2", process_part_1(INPUT));
    }

    #[test]
    #[ignore]
    fn test_process_part_2() {
        assert_eq!("4", process_part_2(INPUT));
    }
}
