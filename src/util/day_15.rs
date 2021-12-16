use std::collections::{BinaryHeap};
use std::cmp::{Ordering};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

struct Grid {
    data:Vec<Vec<u8>>,
    height:usize,
    width:usize,
}


impl Grid {
    fn new(lines:Vec<&str>, mult:usize) -> Grid {
        let mut grid = vec![];
        let height = lines.len();
        let width = lines.first().unwrap().len();

        for y in 0..lines.len() * mult {
            let line = lines.get(y % height).unwrap();
            let tile_no_y = y / height;

            let mut v = vec![];
            for x in 0..line.len() * mult {
                let cost = line.as_bytes()[x % width] - 0x30;
                let tile_no_x = x / width;
                let risk_incr = tile_no_y + tile_no_x;
                let risk = 1 + (risk_incr as u8 + cost - 1) % 9;
                v.push(risk);
            }
            grid.push(v);
        }

        Grid{data:grid,height:height*mult,width:width*mult }
    }

    fn get_cost(&self, x:usize, y:usize) -> usize {
        *self.data.get(y).unwrap().get(x).unwrap() as usize
    }

    fn get_adjacents(&self, x:usize, y:usize) -> Vec<(usize,usize,usize)> {
        let mut v = vec![];
        if x > 0 {
            v.push((x-1,y))
        }
        if x < self.width-1 {
            v.push((x+1,y))
        }
        if y > 0 {
            v.push((x,y-1));
        }
        if y < self.height-1 {
            v.push((x,y+1))
        }

        v.iter().map(|(x,y)| (*x,*y,self.get_cost(*x,*y))).collect()
    }
}


#[derive(Eq,PartialEq)]
struct Node {
    cost:usize,
    x:usize,
    y:usize
}



impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




fn find_path(grid:&Grid) -> usize {
    let end_x = grid.width - 1;
    let end_y = grid.height - 1;

    let start_node = Node{cost:0,x:0,y:0};
    let mut visited = vec![std::usize::MAX; grid.width * grid.height];
    let mut pq = BinaryHeap::new();

    pq.push(start_node);

    while !pq.is_empty() {
        let current_node = pq.pop().unwrap();
        let current_cost = current_node.cost;
        let x = current_node.x;
        let y = current_node.y;

        if x == end_x && y == end_y {
            return current_cost;
        }

        let prev_visit = visited.get_mut(y * grid.width + x).unwrap();

        // Is this a closer path
        if current_cost < *prev_visit {
            *prev_visit = current_cost;
        } else {
            // This path is longer.. skip
            continue;
        }

        // Get neighbouring nodes...
        let adjacent = grid.get_adjacents(x, y);
        for (x1,y1,cost) in adjacent {
            if let Some(next_cost) = visited.get_mut(y1 * grid.width + x1) {
                if (current_cost + cost) < *next_cost {
                    pq.push(Node{cost:current_cost + cost, x:x1, y:y1});
                }
            } else {
                pq.push(Node{cost:current_cost + cost, x:x1, y:y1});
            }
        }
    }

    panic!("No solution...")
}



fn part1(lines:Vec<&str>) -> String {
    let grid = Grid::new(lines, 1);
    let result = find_path(&grid);

    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let grid = Grid::new(lines, 5);

    let result = find_path(&grid);

    result.to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!("40", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_15.txt");

        assert_eq!("769", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!("315", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_15.txt");

        assert_eq!("2963", solve(input.to_string(), Part2));
    }

}
