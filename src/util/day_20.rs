use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.split("\n\n")
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


struct Image {
    width:i32,
    height:i32,
    infinite_pixel:bool,
    pixels:Vec<Vec<bool>>,
}

impl Image {
    fn parse(input:&str) -> Image {
        let mut pixels = vec![];
        let lines:Vec<&str> = input.lines().collect();
        for y in 0..lines.len() {
            let mut image_col = vec![];
            let line = lines.get(y).unwrap();
            for ch in line.chars() {
                image_col.push(
                    match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid input"),
                    }
                )
            }
            pixels.push(image_col);
        }
        let height = pixels.len() as i32;
        let width = pixels.first().unwrap().len() as i32;
        Image{infinite_pixel:false,pixels,height, width}
    }

    fn generate_next(&self, algo:&str) -> Image {
        let growth = 2;
        let height = self.height + growth;
        let width = self.width + growth;
        let mut pixels = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut output_row = Vec::with_capacity(width as usize);
            for x in 0..width {
                let input_x = x - growth / 2;
                let input_y = y - growth / 2;

                let pixel = self.get_pixel(input_x, input_y, algo);
                output_row.push(pixel);
            }
            pixels.push(output_row);

        }

        let infinite_pixel = match self.infinite_pixel {
            true => algo.as_bytes()[algo.len()-1] as char == '#',
            false => algo.as_bytes()[0] as char == '#',
        };

        Image{pixels,height, width, infinite_pixel}
    }

    fn count_pixels(&self) -> usize {
        self.pixels.iter().map(|v| v.iter().filter(|&p| *p ).count()).sum()
    }

    fn get_pixel(&self, xp:i32, yp:i32, algo:&str) -> bool {
        let mut algo_offset:usize = 0;
        for y in (yp-1)..=(yp+1) {
            for x in (xp-1)..=(xp+1) {
                // If outside image, check infinite pixel...
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    algo_offset = (algo_offset << 1) + self.infinite_pixel as usize;
                } else {
                    let pixel = *self.pixels.get(y as usize).unwrap().get(x as usize).unwrap() as usize;
                    algo_offset = (algo_offset << 1) + pixel;
                }
            }
        }

        // println!("offset = {}", algo_offset);
        algo.as_bytes()[algo_offset] as char == '#'
    }

}


fn part1(lines:Vec<&str>) -> String {
    let algo:String = lines.get(0).unwrap().chars().filter(|c| *c != '\n').collect();
    let mut next_image = Image::parse(lines.get(1).unwrap());

    for _ in 0..2 {
        next_image = next_image.generate_next(algo.as_str());
    }

    next_image.count_pixels().to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let algo:String = lines.get(0).unwrap().chars().filter(|c| *c != '\n').collect();
    let mut next_image = Image::parse(lines.get(1).unwrap());

    for _ in 0..50 {
        next_image = next_image.generate_next(algo.as_str());
    }

    next_image.count_pixels().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        assert_eq!("35", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_20.txt");
        //5278
        assert_eq!("5081", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!("3351", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_20.txt");

        assert_eq!("15088", solve(input.to_string(), Part2));
    }

}
