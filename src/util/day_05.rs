use std::cmp::{max, min};
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
    for _ in 0..size*size {
        grid.push(0);
    }

    let mut i = 0;

    while i < coordinates.len() {

        let x1 = *coordinates.get( i ).unwrap(); i +=1;
        let y1 = *coordinates.get( i ).unwrap(); i +=1;
        let x2 = *coordinates.get( i ).unwrap(); i +=1;
        let y2 = *coordinates.get( i ).unwrap(); i +=1;

        if x1 == x2 {
            // Vertical line
            let y_max = max(y1, y2);
            let y_min = min(y1, y2);
            for y in y_min..=y_max {
                *grid.get_mut(size*y as usize + x1 as usize).unwrap() += 1;
            }
        } else if y1 == y2 {
            // Horizontal line
            let x_max = max(x1, x2);
            let x_min = min(x1, x2);
            for x in x_min..=x_max {
                *grid.get_mut(size*y1 as usize + x as usize).unwrap() += 1;
            }
        } else if diagonal {
            let len = max(x1,x2) - min(x1,x2);
            let xd = if x2 > x1 {
                1
            } else {
                -1
            };
            let yd = if y2 > y1 {
                1
            } else {
                -1
            };

            let mut x = x1;
            let mut y = y1;
            for _ in 0..=len {
                *grid.get_mut(size*y as usize + x as usize).unwrap() += 1;
                x += xd;
                y += yd;
            }
        }
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
