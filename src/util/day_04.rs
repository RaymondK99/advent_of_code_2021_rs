use std::collections::{HashSet, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let numbers:Vec<&str> = input.split("\n\n")
        .collect();

    match part {
        Part::Part1 => part1(numbers),
        Part::Part2 => part2(numbers)
    }
}

#[derive(Debug)]
struct Board {
    board_numbers:HashSet<u32>,
    rows_and_cols:Vec<HashSet<u32>>
}

impl Board {
    fn new(numbers:Vec<u32>) -> Board {
        let mut rows_and_cols = vec![];

        for x in 0..5 {
            let mut set_row = HashSet::new();
            let mut set_col = HashSet::new();

            for y in 0..5 {
                set_row.insert(*numbers.get(x * 5 + y).unwrap());
                set_col.insert(*numbers.get(y * 5 + x).unwrap());

            }
            rows_and_cols.push(set_row);
            rows_and_cols.push(set_col);
        }

        Board{ board_numbers: numbers.iter().copied().collect(), rows_and_cols}
    }

    fn has_bingo(self:&Board, numbers:&Vec<u32>) -> bool {
        let number_set:HashSet<u32> = numbers.iter().cloned().collect();

        // Does any row or column have bingo?
        self.rows_and_cols.iter()
            .find( |s| s.is_subset(&number_set)).is_some()
    }

    fn get_score(self:&Board, numbers:&Vec<u32>) -> u32 {
        let number_set:HashSet<u32> = numbers.iter().cloned().collect();

        self.board_numbers
            .difference(&number_set)
            .copied()
            .sum::<u32>() * numbers.last().unwrap()

    }
}

fn parse_game_context(list:Vec<&str>) -> (VecDeque<u32>, Vec<Board>) {
    let mut it = list.iter();

    let numbers:VecDeque<u32>  = it.next().unwrap().split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    let boards:Vec<Board> = it.map( |item| Board::new(item.split(|c| c == '\n' || c == ' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect())).collect();

    (numbers, boards)
}

fn part1(list:Vec<&str>) -> String {

    let (mut numbers, boards) = parse_game_context(list);
    let mut drawn_numbers = vec![];

    for _ in 0..numbers.len() {
        drawn_numbers.push(numbers.pop_front().unwrap());

        let winning_board = boards.iter().find(|b| b.has_bingo(&drawn_numbers));

        if winning_board.is_some() {
            return winning_board.unwrap().get_score(&drawn_numbers).to_string()
        }
    }

    panic!("No solution")
}


fn part2(list:Vec<&str>) -> String {

    let (mut numbers, mut boards) = parse_game_context(list);
    let mut drawn_numbers = vec![];

    for _ in 0..numbers.len() {
        drawn_numbers.push(numbers.pop_front().unwrap());

        if boards.len() > 1 {
            // Remove all boards with bingo
            let mut i = 0;
            while i < boards.len() {

                if boards.get(i).unwrap().has_bingo(&drawn_numbers) {
                    // remove board
                    boards.remove(i);
                } else {
                    i +=1;
                }
            }
        } else {
            // Only one board remaining, play it until it has bingo...
            let remaining_board = boards.first().unwrap();
            if remaining_board.has_bingo(&drawn_numbers) {
                return remaining_board.get_score(&drawn_numbers).to_string();
            }
        }
    }

    panic!("No solution")
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!("4512", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_04.txt");

        assert_eq!("33348", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!("1924", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_04.txt");

        assert_eq!("8112", solve(input.to_string(), Part2));
    }

}
