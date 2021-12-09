#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    let mut crabs = input.clone();
    crabs.sort();

    let middle = crabs.len() / 2;

    // https://doc.rust-lang.org/std/primitive.slice.html#method.select_nth_unstable
    let (_, median, _) = crabs.select_nth_unstable(middle);

    let non_mut_median = *median;

    let total_fuel = crabs
        .iter()
        .map(|current_crab_position| (non_mut_median - current_crab_position).abs())
        .sum();

    total_fuel
}

#[aoc(day7, part2)]
fn part2(input: &Vec<i32>) -> i32 {
    let mut crabs = input.clone();
    crabs.sort(); // not sure if it's necessary to sort here, but it makes me feel better.

    // yay for brute forcing
    let min_pos = input.iter().min().unwrap().clone();
    let max_pos = input.iter().max().unwrap().clone();

    let mut fuel_costs: Vec<i32> = Vec::new();

    // for all possible positions
    // TODO: refactor to use some iterator chaining
    for pos in min_pos..=max_pos {
        // keep track of the current positions total fuel costs
        let mut fuel_cost: i32 = 0;

        // let fuel_cost = &crabs.par_iter().fold(0,
        //     |cost, crab| {
        //         let distance = (pos - crab).abs();
        //         cost + ((1 + distance) * distance / 2)
        //     }
        // );

        for crab in &crabs {
            // determine cost per crab
            let distance = (pos - crab).abs();

            let cost = (1 + distance) * distance / 2;
            fuel_cost += cost;
        }

        fuel_costs.push(fuel_cost);
    }
    *fuel_costs.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(37, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(168, part2(&parsed_input))
    }
}
