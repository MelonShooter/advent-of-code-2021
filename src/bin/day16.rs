use std::{num::ParseIntError, str::FromStr};

struct Literal(u64);

#[derive(Copy, Clone)]
enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    IsGreater,
    IsLesser,
    IsEqual,
    Unknown,
}

impl From<u16> for PacketType {
    fn from(num: u16) -> Self {
        match num {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::IsGreater,
            6 => PacketType::IsLesser,
            7 => PacketType::IsEqual,
            _ => PacketType::Unknown,
        }
    }
}

struct Header {
    version: u8,
    type_id: PacketType,
}

enum Packet {
    Operator(Header, Vec<Packet>),
    Literal(Header, Literal),
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Copy, Clone)]
        struct BitParser<'a> {
            rem: u16,
            rem_size: u8,
            string: &'a str,
            bits_read: usize,
        }

        impl<'a> BitParser<'a> {
            fn new(string: &'a str) -> Self {
                Self {
                    rem: 0,
                    rem_size: 0,
                    string,
                    bits_read: 0,
                }
            }

            fn parse_bits(&mut self, bit_ct: u8) -> Result<u16, ParseIntError> {
                self.bits_read += bit_ct as usize;
                let bit_ct = if self.rem_size < bit_ct {
                    bit_ct - self.rem_size
                } else {
                    self.rem_size -= bit_ct;
                    let bits = self.rem >> self.rem_size;
                    self.rem &= (1u16 << self.rem_size) - 1;

                    return Ok(bits);
                };
                let idx = (bit_ct / 4 + (bit_ct % 4).clamp(0, 1)) as usize;
                self.rem_size = (idx * 4) as u8 - bit_ct;
                let bits = u16::from_str_radix(&self.string[..idx], 16)?;
                let needed_bits = (bits >> self.rem_size) | (self.rem << bit_ct);
                self.rem = bits & ((1u16 << self.rem_size) - 1);
                self.string = &self.string[idx..];

                Ok(needed_bits)
            }
        }

        fn parse_packet(bit_parser: &mut BitParser) -> Result<Packet, ParseIntError> {
            let header = Header {
                version: bit_parser.parse_bits(3)? as u8,
                type_id: bit_parser.parse_bits(3)?.into(),
            };

            if let PacketType::Literal = header.type_id {
                let mut literal = 0;

                loop {
                    let literal_chunk = bit_parser.parse_bits(5)? as u64;
                    let literal_binary = literal_chunk & 0b1111;

                    literal = (literal << 4) | literal_binary;

                    if (literal_chunk >> 4) & 1 == 0 {
                        break;
                    }
                }

                Ok(Packet::Literal(header, Literal(literal)))
            } else {
                let mut op_packets = Vec::new();

                if bit_parser.parse_bits(1)? == 0 {
                    let total_sub_packet_size = bit_parser.parse_bits(15)? as usize;
                    let target_bits_read = bit_parser.bits_read + total_sub_packet_size;

                    while bit_parser.bits_read < target_bits_read {
                        op_packets.push(parse_packet(bit_parser)?);
                    }
                } else {
                    let sub_packet_ct = bit_parser.parse_bits(11)?;

                    for _ in 0..sub_packet_ct {
                        op_packets.push(parse_packet(bit_parser)?);
                    }
                }

                Ok(Packet::Operator(header, op_packets))
            }
        }

        Ok(parse_packet(&mut BitParser::new(s))?)
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(header, _) => header.version as u32,
            Packet::Operator(header, subpackets) => {
                let mut sum = header.version as u32;

                for packet in subpackets {
                    sum += packet.version_sum();
                }

                sum
            }
        }
    }

    fn eval(&self) -> u64 {
        fn eval_operator(type_id: PacketType, subpackets: &Vec<Packet>) -> u64 {
            let operands = subpackets.iter().map(Packet::eval);

            match type_id {
                PacketType::Sum => operands.sum(),
                PacketType::Product => operands.product(),
                PacketType::Min => operands.min().expect("Min operator has no operands"),
                PacketType::Max => operands.max().expect("Max operator has no operands"),
                PacketType::IsGreater => (subpackets[0].eval() > subpackets[1].eval()) as u64,
                PacketType::IsLesser => (subpackets[0].eval() < subpackets[1].eval()) as u64,
                PacketType::IsEqual => (subpackets[0].eval() == subpackets[1].eval()) as u64,
                _ => panic!("Disallowed packet type"),
            }
        }

        match self {
            Packet::Literal(_, literal) => literal.0,
            Packet::Operator(header, subpackets) => eval_operator(header.type_id, subpackets),
        }
    }
}

fn main() {
    let packet_str = include_str!("../../inputs/day16.txt").trim_end();
    let packet = packet_str.parse::<Packet>().unwrap();

    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.eval());
}
