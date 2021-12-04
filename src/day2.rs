use std::str::FromStr;

#[derive(Debug)]
enum SubmarineMove {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for SubmarineMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, amount) = s.split_once(' ').unwrap();
        let delta = amount.parse::<i32>().unwrap();

        match instruction {
            "forward" => Ok(Self::Forward(delta)),
            "down" => Ok(Self::Down(delta)),
            "up" => Ok(Self::Up(delta)),
            _ => panic!("Unknown submarine move instruction!"),
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<SubmarineMove> {
    input
        .lines()
        .map(|i| i.trim().parse::<SubmarineMove>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<SubmarineMove>) -> i32 {
    let mut x_position = 0;
    let mut depth = 0;

    for sub_move in input {
        match sub_move {
            SubmarineMove::Forward(delta) => x_position = x_position + delta,
            SubmarineMove::Down(delta) => depth = depth + delta,
            SubmarineMove::Up(delta) => depth = depth - delta,
        }
    }

    x_position * depth
}

#[aoc(day2, part2)]
fn part2(input: &Vec<SubmarineMove>) -> i32 {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for sub_move in input {
        match sub_move {
            SubmarineMove::Forward(delta) => {
                x = x + delta;
                depth = depth + (aim * delta);
            }
            SubmarineMove::Down(delta) => aim = aim + delta,
            SubmarineMove::Up(delta) => aim = aim - delta,
        }
    }

    x * depth
}
