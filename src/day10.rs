use std::collections::VecDeque;

const OPEN_CHARS: &[char; 4] = &['(', '[', '{', '<'];
const CLOSE_CHARS: &[char; 4] = &[')', ']', '}', '>'];

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| {
            l.trim().chars().collect()
        })
        .collect()
}

/// determines if the line is unbalanced and returns the illegal char if it is
/// otherwise, the line is balanced and None is returned
fn has_illegal_char(chars: &Vec<char>) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for c in chars.iter() {
        if OPEN_CHARS.contains(c) {
            stack.push(*c);
        }

        if CLOSE_CHARS.contains(c) {
            if let Some(popped) = stack.pop() {
                let expected_open = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => panic!("unexpected character! => {:?}", c)
                };

                if expected_open != popped {
                    return Some(*c);
                }
            } else {
                // it's valid move on to the next
                continue;
            }
        }
    }
    None
}

/// returns the closing sequence to make the provided
/// line complete
fn get_closure(chars: &Vec<char>) -> VecDeque<char> {
    let mut stack = vec![];
    let mut closing_sequence: VecDeque<char> = VecDeque::new();

    for c in chars.iter() {
        if OPEN_CHARS.contains(c) {
            stack.push(*c);
        }

        if CLOSE_CHARS.contains(c) {
            if let Some(popped) = stack.pop() {
                let expected_open = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => panic!("unexpected character! => {:?}", c)
                };

                if expected_open != popped {
                    panic!("corrupted line! incomplete only please. {:?}", &chars);
                }
            }
        }
    }

    // dbg!(&stack);

    while !stack.is_empty() {
        if let Some(popped) = stack.pop() {
            if OPEN_CHARS.contains(&popped) {
                match popped {
                    '(' => closing_sequence.push_back(')'),
                    '[' => closing_sequence.push_back(']'),
                    '{' => closing_sequence.push_back('}'),
                    '<' => closing_sequence.push_back('>'),
                    _ => panic!("unexpected character! => {:?}", popped)
                }
            }
        }
    }
    closing_sequence
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut illegal_chars: Vec<char> = vec![];

    for line in input.iter() {
        if let Some(illegal_char) = has_illegal_char(line) {
            illegal_chars.push(illegal_char);
        }
    }

    let mut error_score = 0;
    for illegal_char in illegal_chars {
        match illegal_char {
            ')' => error_score += 3,
            ']' => error_score += 57,
            '}' => error_score += 1197,
            '>' => error_score += 25137,
            _ => panic!("unexpected character! => {:?}", illegal_char)
        }
    }

    error_score
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut scores = vec![];

    for line in input.iter() {
        if has_illegal_char(line).is_none() {
            let mut score = 0;
            let closure = get_closure(line);

            for c in closure.iter() {
                // dbg!(&c, &score);
                match c {
                    ')' => score = (score * 5) + 1,
                    ']' => score = (score * 5) + 2,
                    '}' => score = (score * 5) + 3,
                    '>' => score = (score * 5) + 4,
                    _ => panic!("unexpected closing character! => {:?}", c)
                }
            }
            scores.push(score);
        }
    }

    scores.sort();
    let middle = scores.len() / 2;

    scores[middle]
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(26397, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(288957, part2(&parsed_input))
    }
}