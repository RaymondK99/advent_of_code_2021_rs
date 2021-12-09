use std::collections::HashSet;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(lines:Vec<&str>) -> Vec<Vec<i32>> {
    lines.iter()
        .map(|line|
            line.bytes()
                .map(|byte| (byte - 0x30) as i32)
                .collect())
        .collect()
}

fn is_low_point(x:i32,y:i32,grid:&Vec<Vec<i32>>) -> bool {
    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;
    let current_height = grid.get(y as usize).unwrap().get(x as usize).unwrap();

    let positions:Vec<(i32,i32)> = vec![(x-1,y),(x+1,y),(x,y-1),(x,y+1)]
        .iter()
        .filter(|(x,y)| *x >= 0 && *x < width && *y >= 0 && *y < height )
        .copied()
        .collect();

    positions.iter()
        .map(|&(x,y)| grid.get(y as usize).unwrap().get(x as usize).unwrap())
        .all(|&height| height > *current_height)
}

fn get_low_points(grid:&Vec<Vec<i32>>) -> Vec<(i32,i32,i32)>{
    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;

    let positions:Vec<(i32,i32,i32)> = (0..height)
        .into_iter()
        .map(|y| (0..width).into_iter()
            .map(move |x| (x,y, *grid.get(y as usize).unwrap().get(x as usize).unwrap())))
        .flatten()
        .collect();

    positions.iter().filter(|&(x,y,_)| is_low_point(*x,*y,&grid)).copied().collect()
}

fn get_adjacent((x1,y1):&(i32,i32), grid:&Vec<Vec<i32>>) -> Vec<(i32,i32,i32)> {
    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;

    vec![(*x1-1,*y1),(*x1+1,*y1),(*x1,*y1-1),(*x1,*y1+1)].iter()
        .filter(|(x,y)| *x >= 0 && *x < width && *y >= 0 && *y < height )
        .map( |(x,y)| (*x,*y, *grid.get(*y as usize).unwrap().get(*x as usize).unwrap()))
        .collect()
}

fn get_basin_size((x_low,y_low,height_low):&(i32,i32,i32), grid:&Vec<Vec<i32>>) -> usize {
    let mut basin = HashSet::new();
    let mut eval = vec![(*x_low,*y_low, *height_low)];


    while !eval.is_empty() {
        let (x,y, current_height) = eval.pop().unwrap();

        basin.insert( (x,y, current_height));

        let adjacent:Vec<(i32,i32,i32)> = get_adjacent(&(x,y), &grid ).iter()
            .filter(|item| !basin.contains(item))
            .filter(|&(_,_,height)| *height != 9 && *height > current_height)
            .copied()
            .collect();

        adjacent.iter().for_each(|item| {
            basin.insert(item.clone());
            eval.push( item.clone() )
        });
    }

    basin.len()

}

fn part1(lines:Vec<&str>) -> String {
    let grid= parse(lines);
    let low_points = get_low_points(&grid);

    let risk_level:i32 = low_points.iter()
        .map(|&(_,_,low_point)| low_point + 1)
        .sum();

    risk_level.to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let grid = parse(lines);
    let low_points = get_low_points(&grid);

    let mut basin_sizes:Vec<usize> = low_points.iter().map(|p| get_basin_size(p, &grid)).collect();
    basin_sizes.sort_by(|a,b| b.cmp(a));

    // Multiply 3 biggest basins
    (0..3).into_iter()
        .map(|i| basin_sizes.get(i as usize).unwrap())
        .fold(1,|acc, item| *item as usize * acc)
        .to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!("15", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_09.txt");

        assert_eq!("458", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!("1134", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_09.txt");

        assert_eq!("1391940", solve(input.to_string(), Part2));
    }

}
