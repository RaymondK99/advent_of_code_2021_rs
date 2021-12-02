use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let numbers:Vec<u32> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    }
}

fn part1(list:Vec<u32>) -> String {

    list.iter().
        fold((u32::MAX,0), |(last, cnt), curr|
            {
                if *curr > last {
                    (*curr, cnt + 1)
                } else {
                    (*curr, cnt)
                }
            }).1.to_string()
}


fn part2(list:Vec<u32>) -> String {
    let mut cnt = 0;

    for i in 3..list.len() {
        let curr = list[i-2] + list[i-1] + list[i];
        let last = list[i-3] + list[i-2] + list[i-1];

        if curr > last {
            cnt +=1;
        }
    }

    cnt.to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!("7", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("1715", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!("5", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_01.txt");

        assert_eq!("1739", solve(input.to_string(), Part2));
    }

}
