#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|i| i.trim().chars().collect()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Vec<char>>) -> isize {
    let mut gamma_vec: Vec<char> = Vec::new();
    let mut pos: usize = 0;

    let length = input.first().unwrap().len();

    while pos < length {
        let mut ones = 0;
        let mut zeroes = 0;

        for num in input {
            match num[pos] {
                '1' => ones = ones + 1,
                '0' => zeroes = zeroes + 1,
                _ => panic!("not a 1 or 0"),
            }
        }

        if ones > zeroes {
            gamma_vec.push('1');
        } else {
            gamma_vec.push('0');
        }

        pos = pos + 1;
    }

    let gamma_string: String = gamma_vec.iter().collect();
    let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();

    let mut epsilon_vec: Vec<char> = Vec::new();
    for digit in gamma_vec {
        match digit {
            '1' => epsilon_vec.push('0'),
            '0' => epsilon_vec.push('1'),
            _ => panic!("unimplemented"),
        }
    }

    let epsilon_string: String = epsilon_vec.iter().collect();
    dbg!(&epsilon_string);
    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<char>>) -> isize {
    let o2_rating = apply_bit_criteria((&input).to_vec(), 0, Rating::OxygenGenerator);
    let co2_rating = apply_bit_criteria((&input).to_vec(), 0, Rating::CO2Scrubber);
    o2_rating * co2_rating
}

enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

fn apply_bit_criteria(diag_report: Vec<Vec<char>>, position: usize, rating: Rating) -> isize {
    let mut ones = 0;
    let mut zeroes = 0;

    for num in &diag_report {
        match num[position] {
            '1' => ones = ones + 1,
            '0' => zeroes = zeroes + 1,
            _ => panic!("not a 1 or 0"),
        }
    }

    let mut reduced_report: Vec<Vec<char>> = Vec::new();
    match rating {
        Rating::OxygenGenerator => {
            if zeroes > ones {
                for report in &diag_report {
                    if report[position] == '0' {
                        reduced_report.push(report.clone());
                    }
                }
            } else {
                for report in &diag_report {
                    if report[position] == '1' {
                        reduced_report.push(report.clone());
                    }
                }
            }
        }
        Rating::CO2Scrubber => {
            if zeroes > ones {
                for report in &diag_report {
                    if report[position] == '1' {
                        reduced_report.push(report.clone());
                    }
                }
            } else {
                for report in &diag_report {
                    if report[position] == '0' {
                        reduced_report.push(report.clone());
                    }
                }
            }
        }
    }

    if reduced_report.len() == 1 {
        let rating_string: String = reduced_report[0].iter().collect();
        return isize::from_str_radix(&rating_string, 2).unwrap();
    }
    let new_position = position + 1;
    return apply_bit_criteria(reduced_report, new_position, rating);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(198, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(230, part2(&parsed_input))
    }
}
