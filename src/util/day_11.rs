use std::collections::{HashMap, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn do_step(grid:&mut HashMap<(i32,i32),i32>) -> usize {
    let mut  flashes=0;
    let max_x = grid.iter().map(|((x,_),_)| *x).max().unwrap();
    let max_y = grid.iter().map(|((_,y),_)| *y).max().unwrap();

    let mut increments= VecDeque::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            increments.push_back((x as i32,y as i32));
        }
    }

    while !increments.is_empty() {
        let (x,y) = increments.pop_front().unwrap();
        let octupus = grid.get_mut(&(x,y)).unwrap();
        *octupus += 1;
        if *octupus == 10 {
            // Add increment for adjacent
            let adjacent = vec![(x-1, y), (x-1, y-1), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)];

            adjacent.iter().filter(|&pos| grid.contains_key(pos))
                .for_each(|pos| increments.push_back(pos.clone()));
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            let octupus = grid.get_mut(&(x as i32,y as i32)).unwrap();
            if *octupus > 9 {
                *octupus = 0;
                flashes += 1;
            }
        }
    }

    flashes
}



fn parse(lines:Vec<&str>) -> HashMap<(i32,i32),i32> {
    let mut map = HashMap::new();
    lines.iter()
        .enumerate()
        .for_each(|(y,line)| line.bytes().enumerate()
            .for_each(|(x,b)|{
                map.insert((x as i32,y as i32), (b-0x30) as i32);
            } ));

    map
}

fn part1(lines:Vec<&str>) -> String {
    let mut grid = parse(lines);
    let total:usize = (0..100).into_iter().map(|_| do_step(&mut grid)).sum();
    total.to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let mut grid = parse(lines);
    let mut step = 1;

    while do_step(&mut grid) != grid.len() {
        step += 1;
    }

    step.to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};



    #[test]
    fn test1() {

        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!("1656", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_11.txt");

        assert_eq!("1615", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!("195", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_11.txt");

        assert_eq!("249", solve(input.to_string(), Part2));
    }

}
