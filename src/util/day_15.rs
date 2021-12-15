
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Pos {
    x:i32,
    y:i32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Node {
    cost:usize,
    position:Pos
}

impl Pos {

    fn new(x:i32,y:i32) -> Pos {
        Pos{x:x,y:y}
    }

    fn get_adjacent(&self) -> Vec<Pos> {
        vec![Pos::new(self.x-1,self.y), Pos::new(self.x+1,self.y), Pos::new(self.x,self.y-1), Pos::new(self.x,self.y+1)]
    }

}


fn find_path(grid:&HashMap<Pos,usize>) -> usize {
    let end_x = grid.iter().map(|(p,_)| p.x).max().unwrap();
    let end_y = grid.iter().map(|(p,_)| p.y).max().unwrap();
    let end_pos = Pos::new(end_x, end_y);

    let start_node = Node{cost:0, position:Pos::new(0,0)};
    let mut visited = HashMap::new();
    let mut pq = BinaryHeap::new();

    pq.push(Reverse(start_node));

    while !pq.is_empty() {
        let current_node = pq.pop().unwrap().0;

        if current_node.position.eq(&end_pos) {
            return current_node.cost
        }

        if let Some(prev_visit) = visited.get_mut(&current_node.position) {
            // Is this a closer path
            if current_node.cost < *prev_visit {
                visited.insert(current_node.position, current_node.cost);
            } else {
                // This path is longer.. skip
                continue;
            }
        } else {
            visited.insert(current_node.position, current_node.cost);
        }


        // Get neighbouring nodes...
        let adjacent_nodes:Vec<Node> = current_node.position.get_adjacent()
            .iter()
            .map(|pos| (pos, grid.get(pos)))
            .filter(|(_,item)|item.is_some())
            .map(|(pos, cost)|
                Node{cost:*cost.unwrap() + current_node.cost,position:Pos::new(pos.x,pos.y)})
            .collect();

        // Add to prio queue for evaluation
        for n in adjacent_nodes {
            pq.push(Reverse(n));
        }

    }

    panic!("No solution...")
}

fn parse(lines:Vec<&str>, mult:usize) -> HashMap<Pos,usize>{
    let mut map = HashMap::new();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    for y in 0..height*mult {
        let tile_no_y = y / height;
        let line = lines.get( y % height).unwrap();
        for x in 0..width*mult {
            let tile_no_x = x / width;
            let risk_incr = tile_no_y + tile_no_x;
            let risk = *line.as_bytes().get(x % width).unwrap() as usize - 0x30 as usize;
            let mod_risk = if (risk + risk_incr) > 9 {
                (risk_incr + risk) % 9
            } else {
                risk + risk_incr
            };
            map.insert(Pos::new(x as i32,y as i32), mod_risk);
        }
    }

    map
}

fn part1(lines:Vec<&str>) -> String {
    let grid = parse(lines,1);

    let result = find_path(&grid);
    result.to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let grid = parse(lines, 5);

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
