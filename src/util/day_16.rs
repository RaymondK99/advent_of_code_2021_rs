use super::Part;
use super::packet_computer::PacketComputer;


pub fn solve(input : String, part: Part) -> String {

    match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    }
}



fn part1(line:&str) -> String {
    let mut packet_computer = PacketComputer::new(line);
    packet_computer.run();
    packet_computer.version_sum.to_string()
}


fn part2(line:&str) -> String {
    let mut packet_computer = PacketComputer::new(line);
    packet_computer.run().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "D2FE28";

        assert_eq!("6", solve(input.to_string(), Part1));
    }

    #[test]
    fn test11() {

        let input = "38006F45291200";
        assert_eq!("9", solve(input.to_string(), Part1));
    }

    #[test]
    fn test12() {

        let input = "EE00D40C823060";

        assert_eq!("14", solve(input.to_string(), Part1));
    }


    #[test]
    fn test13() {

        let input1 = "8A004A801A8002F478";
        let input2 = "620080001611562C8802118E34";
        let input3 = "C0015000016115A2E0802F182340";
        let input4 = "A0016C880162017C3686B18A3D4780";

        assert_eq!("16", solve(input1.to_string(), Part1));
        assert_eq!("12", solve(input2.to_string(), Part1));
        assert_eq!("23", solve(input3.to_string(), Part1));
        assert_eq!("31", solve(input4.to_string(), Part1));

    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input_16.txt");

        assert_eq!("901", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input1 = "C200B40A82";
        let input2 = "04005AC33890";
        let input3 = "880086C3E88112";
        let input4 = "CE00C43D881120";
        let input5 = "D8005AC2A8F0";
        let input6 = "F600BC2D8F";
        let input7 = "9C005AC2F8F0";
        let input8 = "9C0141080250320F1802104A08";

        assert_eq!("3", solve(input1.to_string(), Part2));
        assert_eq!("54", solve(input2.to_string(), Part2));
        assert_eq!("7", solve(input3.to_string(), Part2));
        assert_eq!("9", solve(input4.to_string(), Part2));
        assert_eq!("1", solve(input5.to_string(), Part2));
        assert_eq!("0", solve(input6.to_string(), Part2));
        assert_eq!("0", solve(input7.to_string(), Part2));
        assert_eq!("1", solve(input8.to_string(), Part2));

    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_16.txt");

        assert_eq!("110434737925", solve(input.to_string(), Part2));
    }

}
