use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn translate_digit(digit:&[char], translation:&[char]) -> Vec<char> {
    let chars = vec!['a','b','c','d','e','f','g'];
    let mut result = vec![];
    for ch in digit {
        let pos = translation.iter().enumerate().find(|&(_,c)| *c == *ch).unwrap().0;
        result.push(*chars.get(pos).unwrap());
    }
    result.sort_unstable();
    result


}

fn from_lcd_digit(digit:&[char], translation:&[char]) -> u32 {
    let digit:String = translate_digit(digit, translation).iter().copied().collect();

    match digit.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!(),
    }
}

fn get_lcd_digit(digit:u32, translation:&[char]) -> Vec<char> {

    let digit = match digit {
        0 => "abcefg",
        1 => "cf",
        2 => "acdeg",
        3 => "acdfg",
        4 => "bcdf",
        5 => "abdfg",
        6 => "abdefg",
        7 => "acf",
        8 => "abcdefg",
        9 => "abcdfg",
        _ => panic!(".."),
    };

    let indexes:Vec<usize> = digit.chars().map(|ch| (ch as usize - 'a' as usize) as usize).collect();
    let mut translated_digit:Vec<char> = indexes.iter()
        .map( |index| translation.get( *index).unwrap())
        .copied()
        .collect();

    translated_digit.sort_unstable();
    translated_digit
}

fn generate_permutation(translation:&[char]) -> Vec<Vec<char>>{
    let mut lcd_digits = vec![];
    for n in 0..10 {
        let lcd_digit = get_lcd_digit(n, translation);
        lcd_digits.push(lcd_digit);
    }
    lcd_digits
}



fn parse(line:&str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut it  = line.split('|');
    let signals = it.next().unwrap().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut sorted:Vec<char> = s.chars().collect();
            sorted.sort_unstable();
            sorted
        })
        .collect();

    let read_numbers = it.next().unwrap().split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut sorted:Vec<char> = s.chars().collect();
            sorted.sort_unstable();
            sorted
        })
        .collect();

    (signals, read_numbers)
}

fn part1(lines:Vec<&str>) -> String {

    let count = lines.iter()
        .map(|s| s.split('|').last().unwrap())
        .map(|s| s.split(' ')).flatten()
        .filter(|s| !s.is_empty())
        .filter(|s| vec![2,3,4,7].contains(&(s.len() as i32)))
        .count();

    count.to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let chars:Vec<char> = "abcdefg".chars().collect();
    let mut sum = 0;
    let mut permutation_context = vec![];

    for permutation in permute::permute(chars) {
        let lcd_digits = generate_permutation(&permutation);
        permutation_context.push((permutation, lcd_digits));
    }

    for line in lines {
        let (signal_pattern, output) = parse(line);
        for permutation in permutation_context.iter() {
            let (translation, lcd_digits) = permutation;
            if signal_pattern.iter().all(|signal| lcd_digits.contains(signal)) {
                // Found permutation..
                let res: u32 = output.iter()
                    .map(|digit| from_lcd_digit( digit, &translation))
                    .fold(0, |acc, next| acc * 10 + next);

                sum += res;
            }
        }
    }


    sum.to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce";

        assert_eq!("26", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_08.txt");

        assert_eq!("355", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!("61229", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_08.txt");

        assert_eq!("983030", solve(input.to_string(), Part2));
    }

}
