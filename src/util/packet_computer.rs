use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct PacketComputer {
    bits:VecDeque<bool>,
    offset:usize,
    pub(crate) version_sum:u64,
}

#[derive(Debug, PartialEq)]
enum TypeID {
    SUM = 0,
    PRODUCT = 1,
    MIN = 2,
    MAX = 3,
    LITERAL = 4,
    GreatherThan = 5,
    LessThan = 6,
    EQUAL = 7,
}

#[derive(Debug, PartialEq)]
enum LengthId {
    NumberOfBits = 0,
    NumberOfPackets = 1,
}

trait Packet {
    fn process(&self) -> u64;
}

struct LiteralPacket {
    value:u64,
}

struct OperatorPacket {
    sub_packets:Vec<Box<dyn Packet>>,
    type_id:TypeID,
}

impl Packet for LiteralPacket {
    fn process(&self) -> u64 {
        self.value
    }
}

impl Packet for OperatorPacket {
    fn process(&self) -> u64 {
        let values:Vec<u64> = self.sub_packets.iter()
            .map(|sub| sub.process()).collect();

        match self.type_id {
            TypeID::SUM => values.iter().fold(0, |acc,it| acc+it),
            TypeID::PRODUCT => values.iter().fold(1, |acc,it| acc * it),
            TypeID::MIN => *values.iter().min().unwrap(),
            TypeID::MAX => *values.iter().max().unwrap(),
            TypeID::GreatherThan => {
                if values[0] > values[1] {
                    1
                } else{
                    0
                }
            },
            TypeID::LessThan => {
                if values[0] < values[1] {
                    1
                } else{
                    0
                }
            },
            TypeID::EQUAL => {
                if values[0] == values[1] {
                    1
                } else{
                    0
                }
            },
            _ => panic!("not impl"),
        }
    }
}

impl PacketComputer {
    pub(crate) fn new(line:&str) -> PacketComputer {
        let bits: VecDeque<bool> = line
            .chars()
            .map(|ch| u8::from_str_radix(&ch.to_string(), 16).unwrap())
            .map(|byte| {
                let mut nibble = vec![];
                for bit_no in 0..4 {
                    nibble.push(byte >> bit_no & 1 > 0);
                }
                nibble.reverse();
                nibble
            })
            .flatten()
            .collect();

        PacketComputer {bits,offset:0, version_sum:0}
    }



    fn process_literal(&mut self) -> Box<dyn Packet> {
        let (_no_bits, literal_value) = self.read_literal();
        //println!("   => processed literal of size:{}, value:{}", no_bits, literal_value);
        Box::new(LiteralPacket{value:literal_value})
    }

    fn process_operator(&mut self, type_id:TypeID) -> Box<dyn Packet> {
        let length_id = self.read_length_type();
        let sub_packets = if length_id == LengthId::NumberOfBits {
            let no_bits = self.read_bits(15) as usize;
            //println!("  ==> Process sub packets: no_bits:{}", no_bits);
            self.process_sub_packet_bits(no_bits)
        } else {
            let no_packets = self.read_bits(11);
            (0..no_packets).into_iter().map(|_| self.process_packet()).collect()
        };

        Box::new(OperatorPacket{sub_packets, type_id })
    }

    fn process_packet(&mut self) -> Box<dyn Packet> {
        let _version = self.read_version();
        let type_id = self.read_type();

        //println!("=> Found SUB packet version:{}, type={:?}", version, type_id);
        match type_id {
            TypeID::LITERAL => self.process_literal(),
            _ => self.process_operator(type_id)
        }
    }

    fn process_sub_packet_bits(&mut self, len:usize) -> Vec<Box<dyn Packet>> {
        let offset = self.offset;
        let mut sub_packets: Vec<Box<dyn Packet>> = vec![];
        while offset + len > self.offset {
            sub_packets.push(self.process_packet());
        }

        sub_packets
    }

    pub(crate) fn run(&mut self) -> u64 {
        let packet = self.process_packet();
        packet.process()
    }

    fn read_bits(&mut self, no_bits:usize) -> u64 {
        let mut result = 0;
        for n in 0..no_bits {
            let bit = self.bits.pop_front().unwrap() as u32;
            let bit_pos = no_bits - 1 - n;
            result = result | (bit << bit_pos);
            self.offset += 1;
        }
        result as u64
    }

    fn read_version(&mut self) -> u64 {
        let version = self.read_bits(3);
        self.version_sum += version;
        version
    }

    fn read_type(&mut self) -> TypeID {

        match self.read_bits(3) {
            0 => TypeID::SUM,
            1 => TypeID::PRODUCT,
            2 => TypeID::MIN,
            3 => TypeID::MAX,
            4 => TypeID::LITERAL,
            5 => TypeID::GreatherThan,
            6 => TypeID::LessThan,
            7 => TypeID::EQUAL,
            _ => panic!("unknown operator"),
        }
    }

    fn read_length_type(&mut self) -> LengthId {
        match self.read_bits(1) {
            0 => LengthId::NumberOfBits,
            1 => LengthId::NumberOfPackets,
            _ => panic!("invalid length id")
        }
    }

    fn read_literal(&mut self) -> (usize,u64) {
        let mut flag = self.read_bits(1);
        let mut value = self.read_bits(4);
        let mut no_bits = 4;
        while flag == 1 {
            flag = self.read_bits(1);
            let next_value = self.read_bits(4);
            value = (value << 4) + next_value;
            no_bits += 4;
        }

        (no_bits, value)
    }
}