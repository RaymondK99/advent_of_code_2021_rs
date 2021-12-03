use std::mem::size_of;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn count_ones(lines:&Vec<&str>, pos:usize) -> usize {
    lines.iter().filter( |&line| *line.as_bytes().get(pos).unwrap() == 0x31 ).count()
}

fn part1(lines:Vec<&str>) -> String {
    let width = lines.iter().next().unwrap().len();
    let no_lines = lines.len();

    let gamma:u32 = (0..width)
        .map(|pos| count_ones(&lines, pos))
       .rev().enumerate()
        .map(|(index, cnt)|
            if cnt > no_lines/2 {
                1 << index
            } else {
                0
            })
        .sum();

    let epsilon = !gamma & ((1 << width) - 1);

    (gamma * epsilon).to_string()
}


fn fold_to_value(line:&str) -> u32 {
    line.as_bytes().iter().rev().enumerate().map(|(i,b)| ((*b - 0x30) as u32) << i as u32).sum()
}

fn filter_value(lines:&mut Vec<&str>, pos:usize, filter_ones:bool) {
    let mut i = 0;
    while i < lines.len() {
        let line = *lines.get(i).unwrap();
        let value = *line.as_bytes().get(pos).unwrap() - 0x30;

        if filter_ones && value != 1 || !filter_ones && value != 0 {
            lines.remove(i);
        } else {
            i += 1;
        }
    }
}

fn reduce(mut lines: Vec<&str>, filter_ones:bool) -> u32 {
    let width = lines.iter().next().unwrap().len();
    let mut i = 0;

    while lines.len() > 1 && i < width {
        let ones = count_ones(&lines, i);
        let zeroes = lines.len() - ones;

        if ones >= zeroes {
            filter_value(&mut lines, i, filter_ones);
        } else {
            filter_value(&mut lines, i, !filter_ones);
        }

        i +=1;
    }

    fold_to_value(lines.first().unwrap())
}

fn part2(mut lines:Vec<&str>) -> String {
    let oxygen_gen_rating = reduce(lines.clone(), true);
    let c02_scrubber_rating = reduce(lines, false);

    (oxygen_gen_rating * c02_scrubber_rating).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!("198", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_03.txt");

        assert_eq!("3009600", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!("230", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_03.txt");

        assert_eq!("6940518", solve(input.to_string(), Part2));
    }

}
