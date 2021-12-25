use util::day_25::Point::{Down, Empty, Right};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Point {
    Empty,
    Right,
    Down,
}

struct Grid {
    points:Vec<Vec<Point>>,
}

impl Point {
    fn from(ch:char) -> Point {
        match ch {
            '>' => Point::Right,
            'v' => Point::Down,
            _ => Point::Empty,
        }
    }
}

impl Grid {

    fn from(lines:Vec<&str>) -> Grid {
        let mut points = vec![];
        for line in lines {
            points.push(line.chars().map(|ch| Point::from(ch)).collect());
        }
        Grid{points}
    }

    fn move_cucumbers_right(&mut self) -> bool {
        let mut moves = vec![];
        for y in 0..self.points.len() {
            let line = self.points.get_mut(y).unwrap();
            for x in 0..line.len() {
                if *line.get(x).unwrap() == Point::Right {
                    // check next
                    let next_x = if x <= (line.len() - 2) {
                        x+1
                    } else {
                        0
                    };

                    // Is next clear
                    if *line.get(next_x).unwrap() == Point::Empty {
                        moves.push((x,y,next_x));
                    }
                }
            }
        }

        if moves.is_empty() {
            return false;
        } else {
            for (x, y, next_x) in moves {
                *self.points.get_mut(y).unwrap().get_mut(x).unwrap() = Empty;
                *self.points.get_mut(y).unwrap().get_mut(next_x).unwrap() = Right;
            }
            return true;
        }
    }

    fn move_cucumbers_down(&mut self) -> bool {
        let mut moves = vec![];
        let height = self.points.len();
        for y in 0..self.points.len() {
            let line = self.points.get(y).unwrap();
            for x in 0..line.len() {
                if *line.get(x).unwrap() == Point::Down {
                    // check next
                    let next_y = if y <= (height - 2) {
                        y+1
                    } else {
                        0
                    };

                    // Is next clear
                    if *self.points.get(next_y).unwrap().get(x).unwrap() == Point::Empty {
                        moves.push((x,y,next_y));
                    }
                }
            }
        }

        if moves.is_empty() {
            return false;
        } else {
            for (x,y, next_y) in moves {
                *self.points.get_mut(y).unwrap().get_mut(x).unwrap() = Empty;
                *self.points.get_mut(next_y).unwrap().get_mut(x).unwrap() = Down;
            }
            return true;
        }
    }


    fn step(&mut self) -> bool {
        let right = self.move_cucumbers_right();
        let down = self.move_cucumbers_down();
        right || down
    }

}

fn part1(lines:Vec<&str>) -> String {
    let mut grid = Grid::from(lines);
    let mut step = 1;
    loop {
        if !grid.step() {
            println!("step={}", step);
            break;
        } else {
            step += 1;
        }

    }

    step.to_string()
}


fn part2(_lines:Vec<&str>) -> String {
    "1".to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

        assert_eq!("58", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_25.txt");

        assert_eq!("334", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";
        assert_eq!("1", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_25.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }

}
