#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    let mut increases = 0;
    for pair in input.windows(2) {
        if pair[1] > pair[0] {
            increases = increases + 1;
        }
    }
    increases
}

#[aoc(day1, part2)]
fn part2(input: &Vec<i32>) -> usize {
    input
        .windows(3)
        .map(|t| t.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(7, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(5, part2(&parsed_input))
    }
}
