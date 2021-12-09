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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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
}
