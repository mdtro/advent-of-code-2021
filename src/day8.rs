use std::collections::{HashMap, HashSet};

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.trim().split_once("|").unwrap();
            (left.trim().to_string(), right.trim().to_string())
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<(String, String)>) -> usize {
    let mut digit_lengths = vec![];

    for (_, digits) in input.iter() {
        for digit in digits.split(' ') {
            digit_lengths.push(digit.len())
        }
    }

    digit_lengths
        .into_iter()
        .filter(|length| *length == 2 || *length == 4 || *length == 3 || *length == 7)
        .count()

    // DIGIT    =>  SEGMENTS ON
    // 1            2
    // 4            4
    // 7            3
    // 8            7
}

#[aoc(day8, part2)]
fn part2(input: &Vec<(String, String)>) -> i32 {
    // DIGIT    =>  SEGMENTS ON
    // 0            6 (contains #1, but not 9) [round 3]
    // 1            2 (given) [round 1]
    // 2            5 (do this one last)
    // 3            5 (not #1) [round 2]
    // 4            4 (given) [round 1]
    // 5            5 (contained in 6)
    // 6            6 (figure out 1 and 9 first) [round 4]
    // 7            3 (given) [round 1]
    // 8            7 (given) [round 1]
    // 9            6 (contains #4) [round 2]

    // [1, 4, 7, 8]
    // [9]
    // [3, 0]
    // [6]
    // [5]
    // [2]

    let mut total_sum = 0;

    for (segments, digits) in input {
        let mut wire_mapping_to_digit: HashMap<char, HashSet<char>> = HashMap::new();
        let mut display: Vec<char> = vec![];

        while wire_mapping_to_digit.len() < 9 {
            //dbg!(&wire_mapping_to_digit.len());
            //dbg!(&round);
            //dbg!(&wire_mapping_to_digit);
            for segment in segments.split(' ') {
                let wire: HashSet<char> = segment.chars().collect();

                // round 1
                // find the givens: 1, 4, 7, 8
                if segment.len() == 2 {
                    wire_mapping_to_digit.insert('1', wire.clone());
                } else if segment.len() == 4 {
                    wire_mapping_to_digit.insert('4', wire.clone());
                } else if segment.len() == 3 {
                    wire_mapping_to_digit.insert('7', wire.clone());
                } else if segment.len() == 7 {
                    wire_mapping_to_digit.insert('8', wire.clone());
                }

                // round 2
                // find 9
                if let Some(mapping_4) = wire_mapping_to_digit.get(&'4') {
                    if segment.len() == 6 && wire.is_superset(&mapping_4) {
                        wire_mapping_to_digit.insert('9', wire.clone());
                    }
                }

                // should also be round 2
                // find 3
                if let Some(mapping_1) = wire_mapping_to_digit.get(&'1') {
                    if segment.len() == 5 && wire.is_superset(&mapping_1) {
                        wire_mapping_to_digit.insert('3', wire.clone());
                    }
                }

                // round 3
                // find 0
                if let (Some(mapping_9), Some(mapping_1)) = (
                    wire_mapping_to_digit.get(&'9'),
                    wire_mapping_to_digit.get(&'1'),
                ) {
                    if segment.len() == 6
                        && !wire.is_superset(&mapping_9)
                        && wire.is_superset(&mapping_1)
                    {
                        wire_mapping_to_digit.insert('0', wire.clone());
                    }
                }

                // at this point we know 1, _ , 3, 4, _, _, 7, 8, 9

                // round 4
                // find 6
                if let (Some(mapping_9), Some(mapping_1)) = (
                    wire_mapping_to_digit.get(&'9'),
                    wire_mapping_to_digit.get(&'1'),
                ) {
                    if segment.len() == 6
                        && !wire.is_superset(&mapping_1)
                        && !wire.is_superset(&mapping_9)
                    {
                        wire_mapping_to_digit.insert('6', wire.clone());
                    }
                }

                // round 5?
                // find 5
                if let Some(mapping_6) = wire_mapping_to_digit.get(&'6') {
                    if segment.len() == 5 && wire.is_subset(&mapping_6) {
                        wire_mapping_to_digit.insert('5', wire.clone());
                    }
                }

                // last round, by deduction we know 2

                // if let (Some(mapping_6), Some(mapping_5), Some(mapping_3)) = (
                //     wire_mapping_to_digit.get(&'6'),
                //     wire_mapping_to_digit.get(&'5'),
                //     wire_mapping_to_digit.get(&'3'),
                // ) {
                //     if segment.len() == 5 && wire_mapping_to_digit.len() == 9 {
                //         wire_mapping_to_digit.insert('2', wire.clone());
                //     }
                // }
            }
        }

        for digit in digits.split(' ') {
            // this seems very inefficient...

            let digit_chars: HashSet<char> = digit.chars().collect();

            // find the number where all of the characters are matched
            // and append it onto the display
            let mut found = false;
            for mapping in &wire_mapping_to_digit {
                if mapping.1 == &digit_chars {
                    // println!("{} => {:?} (num {})", &digit, &mapping.1, &mapping.0);
                    display.push(mapping.0.clone());
                    found = true;
                }
            }

            if !found {
                display.push('2');
            }
        }

        let display_value = display.into_iter().collect::<String>();
        total_sum += display_value.parse::<i32>().unwrap();
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(26, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(61229, part2(&parsed_input))
    }
}
