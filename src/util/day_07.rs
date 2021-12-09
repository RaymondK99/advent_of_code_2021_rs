use std::cmp::{max, min};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<usize> = input.split(',')
        .map(|s|s.parse().unwrap())
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn calc_pos(numbers:&Vec<usize>) -> Vec<usize> {
    let max = numbers.iter().max().copied().unwrap();
    let mut deltas = Vec::with_capacity(max as usize);
    (0..=max).into_iter().for_each(|_| deltas.push(0));
    numbers.iter().for_each(|num| *deltas.get_mut(*num as usize).unwrap() += 1);
    deltas
}

fn calc(positions:&Vec<usize>,  pos: usize, part_two:bool) -> usize {
    let mut fuel= 0;
    let mut index = pos;

    for i in 0..positions.len() {
        let dist = max(pos,i) - min(pos, i);
        if !part_two {
            fuel += dist * positions.get(i).unwrap();
        } else {
            //  (N*(N + 1)) / 2
            let fuel_cost = (dist * (dist + 1)) / 2;
            fuel += fuel_cost * positions.get(i).unwrap();
        }
        index = (index + 1) % positions.len();
    }
    fuel
}


fn part1(numbers:Vec<usize>) -> String {
    let deltas = calc_pos(&numbers);
    let min_fuel= (0..deltas.len()).
        into_iter()
        .map(|pos | calc(&deltas, pos, false))
        .min().unwrap();

    min_fuel.to_string()
}


fn part2(numbers:Vec<usize>) -> String {
    let deltas = calc_pos(&numbers);
    let min_fuel= (0..deltas.len()).into_iter()
        .map(|pos | calc(&deltas, pos, true))
        .min()
        .unwrap();

    min_fuel.to_string()}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!("37", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_07.txt");
        assert_eq!("356958", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!("168", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_07.txt");
        assert_eq!("105461913", solve(input.to_string(), Part2));
    }

}
