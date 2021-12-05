use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

impl Line {
    fn points_of_line(&self) -> Vec<Point> {
        let mut points = vec![];
        match self.direction {
            Direction::Horizontal => {
                let y = self.start.y.clone();

                for x in self.start.x.min(self.end.x)..=self.end.x.max(self.start.x) {
                    let point = Point { x, y };
                    points.push(point);
                }
            }
            Direction::Vertical => {
                let x = self.start.x.clone();
                for y in self.start.y.min(self.end.y)..=self.end.y.max(self.start.y) {
                    let point = Point { x, y };
                    points.push(point);
                }
            }
            Direction::Diagonal => {
                let x_slope = if self.start.x < self.end.x { 1 } else { -1 };
                let y_slope = if self.start.y < self.end.y { 1 } else { -1 };

                for d in 0..=(self.end.x - self.start.x).abs() {
                    let point = Point {
                        x: self.start.x + d * x_slope,
                        y: self.start.y + d * y_slope,
                    };
                    points.push(point);
                }
            }
        }
        points
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_coord, end_coord) = s.split_once("->").unwrap();
        let start: Vec<&str> = start_coord.split(',').map(|coord| coord.trim()).collect();
        let end: Vec<&str> = end_coord.split(',').map(|coord| coord.trim()).collect();

        let direction = if start[0] == end[0] {
            Direction::Vertical
        } else if start[1] == end[1] {
            Direction::Horizontal
        } else {
            Direction::Diagonal
        };

        Ok(Line {
            start: Point {
                x: start[0].parse::<i32>().unwrap(),
                y: start[1].parse::<i32>().unwrap(),
            },
            end: Point {
                x: end[0].parse::<i32>().unwrap(),
                y: end[1].parse::<i32>().unwrap(),
            },
            direction,
        })
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| l.trim().parse::<Line>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<Line>) -> usize {
    // filter for only horizontal and vertical lines
    let mut lines = input.to_vec();
    lines.retain(|line| {
        line.direction == Direction::Horizontal || line.direction == Direction::Vertical
    });

    let mut points: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        let line_points = line.points_of_line();
        for point in line_points {
            let intersections = points.entry(point).or_insert(0);
            *intersections += 1;
        }
    }

    // why do I have to &2 here instead of just 2...
    points
        .into_iter()
        .filter(|(_, num_of_intersections)| num_of_intersections >= &2)
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Vec<Line>) -> usize {
    let mut points: HashMap<Point, i32> = HashMap::new();
    for line in input {
        let line_points = line.points_of_line();
        for point in line_points {
            let intersections = points.entry(point).or_insert(0);
            *intersections += 1;
        }
    }

    points
        .into_iter()
        .filter(|(_, num_of_intersections)| num_of_intersections >= &2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(5, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(12, part2(&parsed_input))
    }

    // The tests below fail because the Vec is not sorted.
    // I might come back later and use a sorted data structure of some sort.
    //
    // #[test]
    // fn test_vertical_line_draw() {
    //     let line = Line {
    //         start: Point { x: 2, y: 2 },
    //         end: Point { x: 2, y: 1 },
    //         direction: Direction::Vertical,
    //     };
    //     let points = line.points_of_line();
    //     let expected_points = vec![Point { x: 2, y: 2 }, Point { x: 2, y: 1 }];
    //     assert_eq!(points, expected_points);
    // }
    //
    // #[test]
    // fn test_horizontal_line_draw() {
    //     let line = Line {
    //         start: Point { x: 9, y: 7 },
    //         end: Point { x: 7, y: 7 },
    //         direction: Direction::Horizontal,
    //     };
    //     let points = line.points_of_line();
    //     let expected_points = vec![
    //         Point { x: 9, y: 7 },
    //         Point { x: 8, y: 7 },
    //         Point { x: 7, y: 7 },
    //     ];
    //     assert_eq!(points, expected_points);
    // }
}
