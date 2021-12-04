use std::str::FromStr;

#[derive(Debug, Clone)]
struct BingoSubsystem {
    numbers: Vec<i32>,
    boards: Vec<Board>
}

impl BingoSubsystem {
    fn play(&mut self) -> i32 {
        // play number
        for num in &self.numbers {
            // mark boards
            for board in self.boards.iter_mut() {
                board.mark_board(num.clone());
                // check if board is a winner
                if board.is_winner() {
                    return board.calculate_score(num.clone());
                }
            }
        }
        unreachable!() // seems like a bad thing to do here...
    }

    fn play_last_winner(&mut self) -> i32 {

        // play number
        for (play_iteration, num) in self.numbers.iter().enumerate() {
            for (_, board) in self.boards.iter_mut().enumerate() {
                // don't play the board if it has already won
                if board.score.is_some() && board.iterations.is_some() {
                    continue;
                }

                // mark the board
                board.mark_board(num.clone());

                // if the board won on this iteration, set it's score and how many iterations
                // it took to win
                if board.is_winner() {
                    let score = board.calculate_score(num.clone());
                    board.score = Some(score);
                    board.iterations = Some(play_iteration);
                }
            }
        }

        // find the board that won last
        //   - filter for boards with a Some(score) (ie. they won)
        //   - filter for the one with the highest play iterations out of the winning boards

        let score = self.boards.iter().filter(|b|
            b.score.is_some()
        ).max_by_key(|b| b.iterations.unwrap()).unwrap().score.unwrap();
        score
    }
}

#[derive(Debug, Clone)]
struct Board {
    _board_index: usize,
    slots: Vec<(i32, bool)>,
    score: Option<i32>,
    iterations: Option<usize>
}

impl Board {
    fn is_winner(&self) -> bool {
        let mut column_trues: [u8; 5] = [0; 5];
        let mut row_trues: [u8; 5] = [0; 5];

        for (index, value) in self.slots.iter().enumerate() {
            let row_id: usize = index / 5;
            let col_id: usize = index % 5;

            if column_trues.into_iter().any(|x| x == 5) ||
                row_trues.into_iter().any(|x| x == 5) {
                break;
            }

            if value.1 {
                column_trues[col_id] = column_trues[col_id] + 1;
                row_trues[row_id] = row_trues[row_id] + 1;
            }
        }

        column_trues.into_iter().any(|x| x == 5) ||
            row_trues.into_iter().any(|x| x == 5)
    }

    fn mark_board(&mut self, number: i32) {
        if self.is_winner() {
            return
        }

        for value in self.slots.iter_mut() {
            if value.0 == number && !value.1 {
                value.1 = true;
            }
        }

        match self.iterations {
            None => self.iterations = Some(1),
            Some(i) => self.iterations = Some(i + 1)
        }
    }

    fn calculate_score(&self, last_number_called: i32) -> i32 {
        let mut unmarked_sum = 0;
        for value in self.slots.iter() {
            if value.1 == false {
                unmarked_sum = unmarked_sum + value.0;
            }
        }
        // self.score = Some(unmarked_sum * last_number_called);
        unmarked_sum * last_number_called
    }
}

impl FromStr for BingoSubsystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.split("\n\n");

        // grab the first line and populate the numbers
        let numbers: Vec<i32> = lines_iter
            .next()
            .unwrap()
            .trim()
            .split(',')
            .into_iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        // build each board
        let mut boards: Vec<Board> = Vec::new();
        for (board_index, board) in lines_iter.enumerate() {
            let board_values: Vec<(i32, bool)> = board.split_whitespace().map(|n| {
                let num = n.trim().parse::<i32>().unwrap();
                (num, false)
            }).collect();
            boards.push(
                Board {
                    _board_index: board_index,
                    slots: board_values,
                    score: None,
                    iterations: None
                }
            );
        }

        Ok(Self {
            numbers,
            boards
        })
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> BingoSubsystem {
    input.parse::<BingoSubsystem>().unwrap()
}

#[aoc(day4, part1)]
fn part1(input: &BingoSubsystem) -> i32 {
    let mut bingo_subsystem= input.to_owned();
    bingo_subsystem.play()
}

#[aoc(day4, part2)]
fn part2(input: &BingoSubsystem) -> i32 {
    let mut bingo_subsystem = input.to_owned();
    bingo_subsystem.play_last_winner()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(4512, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(1924, part2(&parsed_input))
    }
}