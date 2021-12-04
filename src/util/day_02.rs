use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let numbers:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    }
}

fn map_to_pos(line:&str) -> (i32, i32) {
    let mut it = line.split(' ');
    let cmd = it.next().unwrap();
    let value = it.next().unwrap().parse().unwrap();

    match cmd {
        "forward" => (value, 0),
        "up" => (0, -value),
        "down" => (0, value),
        _ => panic!("invalid value"),
    }
}

fn part1(list:Vec<&str>) -> String {
    let position = list.iter()
        .map( |&line| map_to_pos(line))
        .fold((0,0), |(x_acc,y_acc), (x,y)| (x_acc+x, y_acc+y));

    (position.0 * position.1).to_string()
}


fn part2(list:Vec<&str>) -> String {
    let position = list.iter()
        .map( |&line| map_to_pos(line))
        .fold((0,0,0), |(x_acc,y_acc,aim_acc), (x,y)| (x_acc+x, y_acc+aim_acc*x,aim_acc + y));

    (position.0 * position.1).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!("150", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("1383564", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!("900", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_02.txt");

        assert_eq!("1488311643", solve(input.to_string(), Part2));
    }
}
