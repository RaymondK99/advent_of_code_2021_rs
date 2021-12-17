use std::cmp::{max};
use regex::Regex;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    }
}

#[derive(Debug)]
struct Trajectory {
    x_v:i32,
    y_v:i32,
    x:i32,
    y:i32,
}

#[derive(Debug)]
struct Area {
    x1:i32,
    y1:i32,
    x2:i32,
    y2:i32,
}

impl Area {
    fn parse(line:&str) -> Area {
        let re = Regex::new(r"target area: x=(\d*)..(\d*), y=.(\d*)...(\d*)").unwrap();
        for cap in re.captures_iter(line) {
            let (x1,y1,x2,y2) =
                (cap[1].parse().unwrap(),
                    cap[3].parse::<i32>().unwrap() * -1,
                    cap[2].parse().unwrap(),
                    cap[4].parse::<i32>().unwrap() * -1);
            return Area::new(x1,y1,x2,y2)
        }
        panic!("...");
    }
    fn new(x1:i32,y1:i32, x2:i32,y2:i32) -> Area {
        Area{x1,y1,x2,y2}
    }
}


impl Trajectory {
    fn new (x_v:i32,y_v:i32) -> Trajectory {
        Trajectory{x_v, y_v, x:0, y:0}
    }

    fn fire(&mut self, area:&Area) -> Option<i32> {
        let mut max_y = self.y;
        for _ in 0..1000 {
            self.advance();

            max_y = max(max_y, self.y);

            if self.is_inside(area) {
                return Some(max_y);
            }

            if self.x_v == 0 && self.x < area.x1 {
                break
            }

            if self.y < area.y1 || self.x > area.x2 {
                break;
            }
        }

        None
    }

    fn advance(&mut self) {
        self.x += self.x_v;
        self.y += self.y_v;

        if self.x_v > 0 {
            self.x_v -= 1;
        } else if self.x_v < 0 {
            self.x_v += 1;
        }

        self.y_v -= 1;
    }

    fn is_inside(&self, area:&Area) -> bool {
        self.x >= area.x1 && self.x <= area.x2 &&
            self.y >= area.y1 && self.y <= area.y2
    }
}

fn get_max(line:&str) -> Vec<i32> {
    let area = Area::parse(line);
    let mut ys = vec![];

    for x_v in 1..=area.x2 {
        for y_v in area.y1..200 {
            let mut t = Trajectory::new(x_v, y_v);
            if let Some(y) = t.fire(&area) {
                ys.push(y);
            }
        }
    }
    ys
}

fn part1(line:&str) -> String {
    get_max(line).iter().max().unwrap().to_string()
}


fn part2(line:&str) -> String {
    get_max(line).len().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "target area: x=20..30, y=-10..-5";

        assert_eq!("45", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_17.txt");

        assert_eq!("9180", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!("112", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_17.txt");

        assert_eq!("3767", solve(input.to_string(), Part2));
    }

}
