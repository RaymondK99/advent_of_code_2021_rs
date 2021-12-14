use std::collections::{HashSet};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.split("\n\n")
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_paper(lines:Vec<&str>) -> (HashSet<(usize,usize)>, Vec<(usize,usize)>) {
    let mut grid = HashSet::new();
    let mut it = lines.iter();
    it.next().unwrap().lines().for_each(|line| {
        let mut col = line.split(',');
        let x:usize = col.next().unwrap().parse().unwrap();
        let y:usize = col.next().unwrap().parse().unwrap();
        grid.insert((x,y));
    });

    let mut fold_instr = vec![];
    it.next().unwrap().lines().for_each(|line|{
        let fold_param:usize = line.split('=').last().unwrap().parse().unwrap();
        if line.contains("x") {
            fold_instr.push((fold_param, 0));
        } else {
            fold_instr.push((0, fold_param));
        }
    });

    (grid, fold_instr)
}


fn print(grid:&HashSet<(usize,usize)>) -> String {

    let width = grid.iter().map(|(x,_)| *x).max().unwrap() + 1;
    let height = grid.iter().map(|(_,y)| *y).max().unwrap() + 1;
    let mut output:String = String::new();

    for y in 0..height {
        for x in 0..width {
            let point = grid.get(&(x, y));
            if point.is_some() {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    output
}


fn fold_x(grid:&mut HashSet<(usize,usize)>, fold_x:usize) {
    let mut to_remove = vec![];
    let mut to_add = vec![];

    grid.iter().for_each(|(x,y)|{
        if *x > fold_x {
            to_remove.push((*x, *y));
            to_add.push((fold_x * 2 - *x, *y));
        }
    });

    to_remove.iter().for_each(|item| {grid.remove(item);});
    to_add.iter().for_each(|item| {grid.insert(*item);});
}

fn fold_y(grid:&mut HashSet<(usize,usize)>, fold_y:usize) {
    let mut to_remove = vec![];
    let mut to_add = vec![];

    grid.iter().for_each(|(x,y)|{
        if *y > fold_y {
            to_remove.push((*x, *y));
            to_add.push((*x, fold_y * 2 - *y));
        }
    });

    to_remove.iter().for_each(|item| {grid.remove(item);});
    to_add.iter().for_each(|item| {grid.insert(*item);});
}


fn part1(lines:Vec<&str>) -> String {
    let (mut grid, fold_instr) = parse_paper(lines);

    let (fold_x_pos, fold_y_pos) = fold_instr.first().unwrap();
    match fold_x_pos {
        0 => fold_y(&mut grid, *fold_y_pos),
        _ => fold_x(&mut grid, *fold_x_pos),
    }

    grid.len().to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let (mut grid, fold_instr) = parse_paper(lines);

    fold_instr.iter().for_each(|(fold_x_pos, fold_y_pos)|{
        match fold_x_pos {
            0 => fold_y(&mut grid, *fold_y_pos),
            _ => fold_x(&mut grid, *fold_x_pos),
        }
    });

    print!("{}", print(&grid));
    print(&grid)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!("17", solve(input.to_string(), Part1));
    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input_13.txt");
        assert_eq!("695", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = "#####
#   #
#   #
#   #
#####\n";
        assert_eq!(result, solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_13.txt");
        let result = " ##    ## ####  ##  #    #  # ###    ##
#  #    #    # #  # #    #  # #  #    #
#       #   #  #    #    #  # #  #    #
# ##    #  #   # ## #    #  # ###     #
#  # #  # #    #  # #    #  # #    #  #
 ###  ##  ####  ### ####  ##  #     ## \n";
        assert_eq!(result, solve(input.to_string(), Part2));
    }

}
