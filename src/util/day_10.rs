use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn is_open(ch:char) -> bool {
    ['(','[','{','<'].contains(&ch)
}

fn is_close(ch:char) -> bool {
    [')',']','}','>'].contains(&ch)
}

fn get_matching(ch:char) -> char {
    match ch {
        '(' => ')',
        ')' => '(',
        '<' => '>',
        '>' => '<',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        _ => panic!("..."),
    }

}

fn check_line(line:&str) -> (VecDeque<char>, Option<char>) {
    let mut stack = VecDeque::new();
    for ch in line.chars() {
        if is_open(ch) {
            // Open..
            stack.push_front(ch);
            continue;
        } else if is_close(ch) {
            // Close
            if stack.is_empty() {
                return (stack, Option::Some(ch));
            } else {
                // Pop
                let open_char = stack.pop_front().unwrap();
                let expected_closing_char = get_matching(open_char);
                if expected_closing_char != ch {
                    // Unexpected closing char
                    return (stack, Option::Some(ch));
                }
            }
        }
    }

    (stack, None)
}

fn part1(lines:Vec<&str>) -> String {
    let result: Vec<(VecDeque<char>, Option<char>)>= lines.iter().map(|elem| check_line(elem)).collect();

    let sum:usize = result.iter().filter(|(_, ch)| ch.is_some() )
        .map(|(_,ch)| match ch.unwrap() {
            ')' => 3,
            '}' => 1197,
            ']' => 57,
            '>' => 25137,
            _ => panic!("invalid value")
        }).sum();

    sum.to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let rest_list:Vec<VecDeque<char>> = lines.iter()
        .map(|elem| check_line(elem))
        .filter(|(_, ch)| ch.is_none())
        .map(|(rest,_)| rest)
        .collect();

    let mut missing:Vec<u64> = rest_list.iter()
        .map(|rest| rest.iter()
            .map(|ch| get_matching(*ch))
            .map( |ch| {
                match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(".."),
                }
            })
            .fold(0,|acc,value| {
                acc * 5 + value
             })).collect();

    missing.sort();
    missing.get( missing.len() / 2).unwrap().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!("26397", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_10.txt");

        assert_eq!("469755", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!("288957", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_10.txt");

        assert_eq!("2762335572", solve(input.to_string(), Part2));
    }

}
