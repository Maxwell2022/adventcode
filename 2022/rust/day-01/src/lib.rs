pub fn process_part_1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|load| -> i32 {
            load.lines()
                .map(|line| -> i32 { line.parse::<i32>().unwrap() })
                .sum::<i32>()
        })
        .max()
        .unwrap();

    result.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut loads: Vec<_> = input
        .split("\n\n")
        .map(|load| -> i32 {
            load.lines()
                .map(|line| -> i32 { line.parse::<i32>().unwrap() })
                .sum::<i32>()
        })
        .collect();

    loads.sort_by(|a, b| b.cmp(a));

    let result: i32 = loads.iter().take(3).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_process_part_1() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn test_process_part_2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "45000");
    }
}
