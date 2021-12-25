use permute::permute;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.split("\n\n")
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x:i32,
    y:i32,
    z:i32,
}

#[derive(Debug)]
struct Scanner {
    number:usize,
    beacons:Vec<Pos>,
    permutation:usize,
}


impl Pos {

    fn get_permutations(&self) -> Vec<Pos>{
        let initial_state = vec![self.x, self.y, self.z];
        let states = permute(initial_state);
        let directions: Vec<(i32, i32, i32)> = vec![(1, 1, 1), (1, 1, -1), (1, -1, 1), (1, -1, -1), (-1, 1, 1), (-1, 1, -1), (-1, -1, 1), (-1, -1, -1)];

        let mut permutations = vec![];
        for state in states.iter().map(|s| s.as_slice()) {
            for i in 0..directions.len() {
                let (x,y,z) = directions.get(i).unwrap();
                permutations.push(Pos{x:*x*state[0], y:*y*state[1], z:*z*state[2]})
            }
        }

        permutations
    }

    fn _get_permutation(&self,variant:usize) -> Pos {
        let v = self.get_permutations();
        *v.get(variant).unwrap()
    }

    fn _get_scanner_candidate_pos(&self, other:&Pos) -> Pos {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Pos{x,y,z}
    }

}
impl Scanner {

    fn new(lines:&str, number:usize) -> Scanner {
        let mut it = lines.lines();
        let mut beacons = vec![];
        it.next();
        loop {
            if let Some(line) = it.next() {
                let mut cols = line.split(',');
                let x = cols.next().unwrap().parse::<i32>().unwrap();
                let y = cols.next().unwrap().parse::<i32>().unwrap();
                let z = cols.next().unwrap().parse::<i32>().unwrap();
                beacons.push(Pos {x,y,z});
            } else {
                break;
            }
        }

        Scanner{number, beacons,permutation:0}
    }

    fn _next(&mut self) {
        self.permutation = self.permutation + 1;
    }

    fn _get_pos(&self, index:usize) -> Pos {
        self.beacons.get(index).unwrap()._get_permutation(self.permutation)
    }

}



fn match_scanners(_scanner0:&Scanner, _scanner:&mut Scanner) {
    /*
    for pos_scanner0 in scanner0.beacons.iter() {
        for i in 0..24 {
            for index in 0..scanner.beacons.len() {
                let pos = scanner.get_pos(index);
                println!("pos no {}:{:?}", index, pos);

            }
            scanner.next();

        }

    }

     */
    //println!("{:?}", permutations);
}

fn part1(lines:Vec<&str>) -> String {

    let mut scanners:Vec<Scanner> = lines.iter().enumerate().map(|(i,elem)| Scanner::new(elem, i) ).collect();

    match_scanners(&scanners.remove(0), scanners.get_mut(1).unwrap());
    //println!("{:?}", scanners.first().unwrap());
    "1".to_string()
}


fn part2(_lines:Vec<&str>) -> String {
    "1".to_string()
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};



    #[test]
    fn test91() {
        let pos = Pos{x:1,y:2,z:3};

        let p = pos.get_permutations();
        println!("{}, {:?}", p.len(), p);
        let mut s = HashSet::new();
        for n in p {
            s.insert(n);
        }

        println!("{}, {:?}", s.len(), s);


    }
        #[test]
    fn test1() {

        let input = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_19.txt");

        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "";
        assert_eq!("1", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_19.txt");

        assert_eq!("1", solve(input.to_string(), Part2));
    }

}
