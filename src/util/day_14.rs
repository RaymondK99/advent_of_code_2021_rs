use std::collections::{HashMap, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.split("\n\n")
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse(lines:Vec<&str>) -> (VecDeque<char>, Vec<([char;2], char)>) {
    let mut it = lines.iter();
    let template:VecDeque<char> = it.next().unwrap().chars().collect();

    let rules:Vec<([char;2], char)> = it.next().unwrap().lines().map(|line|{
        let mut columns = line.split(" -> ");
        let chars = columns.next().unwrap().as_bytes();
        let pair:[char;2] = [*chars.get(0).unwrap() as char, *chars.get(1).unwrap() as char];
        let insertion:char = *columns.next().unwrap().as_bytes().first().unwrap() as char;

        (pair.clone(), insertion)
    }).collect();

    (template, rules)
}

fn get_rule(a:char, b:char, rules:&Vec<([char;2], char)>) -> char {
    *rules.iter().find( | (chars, _) | chars[0] == a && chars[1] == b )
        .map(|(_,ch)| ch)
        .unwrap()
}


fn count_chars(first:char, next:char, cache:&mut HashMap<(char,char,usize),[u128;30] >, rules:&Vec<([char;2], char)>, step:usize) -> [u128;30] {

    let key = (first, next, step);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let middle = get_rule(first, next, rules);

    if step > 1 {
        let mut count1 = count_chars(first,middle, cache,rules, step - 1);
        let count2 = count_chars(middle,next, cache, rules, step - 1);

        // Cache result
        for i in 0..count2.len() {
            count1[i] += count2[i];
        }
        cache.insert(key, count1);

        return count1;
    } else {
        let mut count:[u128;30] = [0;30];
        count[first as usize - 'A' as usize] += 1;
        count[middle as usize - 'A' as usize] += 1;

        return count;
    }
}

fn calculate_polymer(lines:Vec<&str>, steps:usize) -> (u128, u128) {
    let (mut polymer, rules) = parse(lines);

    let mut count:[u128;30] = [0;30];
    let mut cache = HashMap::new();

    for _ in 0..polymer.len() - 1 {
        let first  = polymer.pop_front().unwrap();
        let second = *polymer.front().unwrap();
        let c = count_chars(first, second, &mut cache, &rules,  steps );
        for i in 0..c.len() {
            count[i] += c[i];
        }
    }

    // Add last char
    let last_char = *polymer.back().unwrap();
    count[last_char as usize - 'A' as usize] += 1;

    let min = *count.iter().filter(|&n| *n > 0).min().unwrap();
    let max = *count.iter().max().unwrap();

    (max, min)
}

fn part1(lines:Vec<&str>) -> String {
    let (max, min) = calculate_polymer(lines, 10);
    (max - min).to_string()
}

fn part2(lines:Vec<&str>) -> String {
   let (max, min) = calculate_polymer(lines, 40);
    (max - min).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!("1588", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_14.txt");

        assert_eq!("2712", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!("2188189693529", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_14.txt");

        assert_eq!("8336623059567", solve(input.to_string(), Part2));
    }

}
