#[aoc_generator(day6)]
fn input_generator(input: &str) -> [usize; 9] {
    let mut counters: [usize; 9] = [0; 9];
    for num_str in input.split(',') {
        let num = num_str.trim().parse::<usize>().unwrap();
        counters[num] += 1;
    }
    counters
}

#[aoc(day6, part1)]
fn part1(input: &[usize; 9]) -> usize {
    let mut fishies = input.clone();

    for _ in 0..80 {
        // https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left ❤️
        fishies.rotate_left(1);
        fishies[6] += fishies[8];
    }

    fishies.iter().sum()
}

#[aoc(day6, part2)]
fn part2(input: &[usize; 9]) -> usize {
    let mut fishies = input.clone();

    for _ in 0..256 {
        fishies.rotate_left(1);
        fishies[6] += fishies[8];
    }

    fishies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(5934, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(26984457539, part2(&parsed_input))
    }
}
