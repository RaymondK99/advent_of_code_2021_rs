use util::day_18::Element::{COMMA, EndBracket, NUMBER, StartBracket};
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
enum Element {
    StartBracket,
    EndBracket,
    COMMA,
    NUMBER(u32),
}

#[derive(Debug)]
struct SnailNumber {
    elements:Vec<Element>,
}

impl SnailNumber {
    fn from(other:&SnailNumber) -> SnailNumber {
        let mut elements = vec![];
        other.elements.iter().for_each(|el| elements.push(*el));
        SnailNumber{elements}
    }

    fn parse(line:&str) -> SnailNumber {
        let mut it = line.chars().into_iter();
        let mut element = it.next();
        let mut elements = vec![];
        while element.is_some() {
            let mut ch = element.unwrap();
            let mut number_str= String::new();
            if ch == '[' {
                elements.push(StartBracket);
            } else if ch == ']' {
                elements.push(EndBracket);
            } else if ch == ',' {
                elements.push(COMMA);
            } else if ch.is_digit(10) {
                while ch.is_digit(10) {
                    number_str.push(ch);
                    element = it.next();
                    ch = element.unwrap();
                }
                elements.push(NUMBER(number_str.parse::<u32>().unwrap()));
                continue;
            }
            element = it.next();
        }
        SnailNumber{elements}
    }

    fn get_number(&self, index:usize) -> Option<u32> {
        let element = self.elements.get(index).unwrap();
        match element {
            NUMBER(n) => Some(*n),
            _ => None,
        }
    }

    fn incr_left(&mut self, mut index:usize, incr:u32)  {
        while index >0 {
            if let Some(number) = self.get_number(index) {
                *self.elements.get_mut(index).unwrap() = NUMBER(incr + number);
                break;
            }
            index -= 1;
        }
    }

    fn incr_right(&mut self, mut index:usize, incr:u32)  {
        while index < self.elements.len() {
            if let Some(number) = self.get_number(index) {
                *self.elements.get_mut(index).unwrap() = NUMBER(incr + number);
                break;
            }
            index += 1;
        }
    }

    fn explode(&mut self) -> bool {
        if let Some(index) = self.get_explode_index() {
            let (left, right) = self.remove_at(index);
            //println!(" ==> Remove ({},{})", left, right);
            self.incr_left(index, left);
            self.incr_right(index, right);
            self.elements.insert(index, NUMBER(0));
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        if let Some(index) = self.get_split_index() {
            //println!("split as index={}, value{:?}",index, self.elements.get(index).unwrap());
            match self.elements.remove(index) {
                NUMBER(num) => {
                    let rest = num % 2;
                    let left = num / 2;
                    let right = num / 2 + rest;
                    self.elements.insert(index, EndBracket);
                    self.elements.insert(index, NUMBER(right));
                    self.elements.insert(index, COMMA);
                    self.elements.insert(index, NUMBER(left));
                    self.elements.insert(index, StartBracket);
                }
                _ => panic!("..."),
            }

            true
        } else {
            false
        }
    }

    fn add(&mut self, other: &mut SnailNumber) {

        if self.elements.is_empty() {
            while !other.elements.is_empty() {
                self.push_back(other.pop_front());
            }
        } else {
            self.push_front(StartBracket);
            self.push_back(COMMA);

            while !other.elements.is_empty() {
                self.push_back(other.pop_front());
            }
            self.push_back(EndBracket);
        }
    }

    fn push_front(&mut self, element:Element) {
        self.elements.insert(0, element);
    }

    fn push_back(&mut self, element:Element) {
        self.elements.push(element);
    }

    fn pop_front(&mut self) -> Element {
        self.elements.remove(0)
    }

    fn reduce(&mut self) {
        loop {
            let mut cnt = 0;
            while self.explode() {
                cnt += 1;
            }

            if self.split() {
                cnt += 1;
            }

            if cnt == 0 {
                break;
            }
        }
    }

    fn remove_at(&mut self,index:usize) -> (u32,u32) {
        self.elements.remove(index); // Start
        let left_number = match self.elements.remove(index) {
            NUMBER(num) => num,
            _ => panic!("..."),
        };
        self.elements.remove(index); // Comma
        let right_number = match self.elements.remove(index) {
            NUMBER(num) => num,
            _ => panic!("..."),
        };
        self.elements.remove(index); // End
        (left_number, right_number)
    }

    fn get_split_index(&self) -> Option<usize> {
        for index in 0..self.elements.len() {
            match *self.elements.get(index).unwrap() {
                NUMBER(num) => {
                    if num > 9 {
                        return Some(index);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn get_explode_index(&mut self) -> Option<usize> {
        let mut depth = 0;
        for index in 0..self.elements.len() {

            match *self.elements.get(index).unwrap() {
                StartBracket => { depth += 1;}
                EndBracket => { depth -= 1; }
                _ => {}
            }
            if depth == 5 && *self.elements.get(index+1).unwrap() == COMMA {
                return Some(index-1);
            }
        }
        None
    }

    fn magnitude(&self) -> u32 {
        if self.elements.len() == 1 {
            return self.get_number(0).unwrap();
        } else if self.elements.len() == 5 {
            let left = self.get_number(1).unwrap();
            let right = self.get_number(3).unwrap();
            return 3*left + 2*right;
        } else {
            let mut depth = 0;
            for i in 0..self.elements.len() {
                let ch = self.elements.get(i).unwrap();
                if *ch == StartBracket {
                    depth += 1;
                } else if *ch == EndBracket {
                    depth -= 1;
                }

                if depth == 1 && *ch == COMMA {
                    // Split
                    let mut left = SnailNumber { elements: vec![] };
                    let mut right = SnailNumber { elements: vec![] };

                    for n in 1..i {
                        left.push_back(*self.elements.get(n).unwrap());
                    }

                    for n in i + 1..self.elements.len() - 1 {
                        right.push_back(*self.elements.get(n).unwrap());
                    }

                    return 3 * left.magnitude() + 2 * right.magnitude();
                }
            }

            panic!("...");
        }

    }
}

fn part1(lines:Vec<&str>) -> String {
    lines.iter()
        .map(|line| SnailNumber::parse(line))
        .fold( SnailNumber{elements:vec![]}, |mut acc, mut elem| {
            acc.add(&mut elem);
            acc.reduce();
            acc })
        .magnitude().to_string()

}


fn part2(lines:Vec<&str>) -> String {

    let numbers:Vec<SnailNumber> = lines.iter().map(|line| SnailNumber::parse(line))
        .collect();

    let mut magnitudes = vec![];

    for i in 0..numbers.len() {
        for n in 0..numbers.len() {
            if i == n {
                continue;
            }

            let mut first = SnailNumber::from(numbers.get(i).unwrap());
            let mut second = SnailNumber::from(numbers.get(n).unwrap());
            first.add(&mut second);
            first.reduce();

            magnitudes.push(first.magnitude());
        }
    }

    magnitudes.iter().max().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    #[test]
    fn test1() {

        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!("4140", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_18.txt");

        assert_eq!("3981", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!("3993", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_18.txt");

        assert_eq!("4687", solve(input.to_string(), Part2));
    }

}
