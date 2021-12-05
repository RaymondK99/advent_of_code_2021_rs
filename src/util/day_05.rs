use std::cmp::Ordering;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn calculate_grid(lines:Vec<&str>, diagonal:bool) -> usize {

    let coordinates:Vec<i32> = lines.iter()
        .map(|line| line.split(|c:char | !c.is_ascii_digit()))
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let size = *coordinates.iter().max().unwrap() as usize + 1;

    let mut grid = Vec::new();
    (0..size*size).into_iter().for_each(|_| grid.push(0));

    for chunk in coordinates.chunks(4) {

        let x1 = chunk[0];
        let y1 = chunk[1];
        let x2 = chunk[2];
        let y2 = chunk[3];

        // Check if diagonal
        if (x1 != x2 && y1 != y2) && !diagonal {
            continue;
        }

        let xd = match x2.cmp(&x1) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            _ => 0
        };

        let yd = match y2.cmp(&y1) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            _ => 0
        };

        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            *grid.get_mut(size*y as usize + x as usize).unwrap() += 1;
            y += yd;
            x += xd;
        }

        *grid.get_mut(size*y as usize + x as usize).unwrap() += 1;
    }

    grid.iter().filter(|&point| *point > 1 ).count()
}

fn part1(lines:Vec<&str>) -> String {
    calculate_grid(lines, false).to_string()
}

fn part2(lines:Vec<&str>) -> String {
    calculate_grid(lines, true).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!("5", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_05.txt");

        assert_eq!("6710", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!("12", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_05.txt");

        assert_eq!("20121", solve(input.to_string(), Part2));
    }
}
