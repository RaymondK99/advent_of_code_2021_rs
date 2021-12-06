use std::collections::HashMap;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let numbers = input.split(',')
        .map(|s|s.parse().unwrap())
        .collect();

    match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    }
}


fn calc_lanterns(age:u32, turns:u32, results: &mut HashMap<(u32, u32), i64>) -> i64 {
    if turns == 0 || turns <= age {
        1
    } else if results.contains_key(&(age, turns)) {
        return *results.get(&(age,turns)).unwrap();
    } else {
        let next_turn = turns - age - 1;
        let count = calc_lanterns(6, next_turn, results) + calc_lanterns(8, next_turn, results);
        results.insert((age, turns), count);
        count
    }
}

fn calculate_lanterns(numbers:Vec<u32>, turns:u32) -> String {
    let mut result_map= HashMap::new();
    numbers.iter()
        .map( |n| calc_lanterns(*n, turns, &mut result_map))
        .sum::<i64>()
        .to_string()
}

fn part1(numbers:Vec<u32>) -> String {
    calculate_lanterns(numbers, 80).to_string()
}

fn part2(numbers:Vec<u32>) -> String {
    calculate_lanterns(numbers, 256).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {
        let input = "3,4,3,1,2";
        assert_eq!("5934", solve(input.to_string(), Part1));
    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input_06.txt");
        assert_eq!("360761", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        let input = "3,4,3,1,2";
        assert_eq!("26984457539", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_06.txt");
        assert_eq!("1632779838045", solve(input.to_string(), Part2));
    }

}
