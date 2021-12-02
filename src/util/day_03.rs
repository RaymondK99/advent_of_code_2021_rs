use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let numbers:Vec<u32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    }
}

fn part1(list:Vec<u32>) -> String {
    "1".to_string()
}


fn part2(list:Vec<u32>) -> String {
    "1".to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "";

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_03.txt");

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";
        assert_eq!("1", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_03.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }

}
