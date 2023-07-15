use super::{Instructions, LogicTargets};

impl Instructions {
    pub fn read_byte(byte: u8, prefixed: bool) -> Option<Instructions> {
        if prefixed {
            Instructions::read_byte_prefixed(byte)
        } else {
            Instructions::read_byte_unprefixed(byte)
        }
    }

    fn read_byte_prefixed(byte: u8) -> Option<Instructions> {
        if byte <= 0x3F {
        match byte {
            0x00 => Some(Instructions::RLC(LogicTargets::B)),
            0x01 => Some(Instructions::RLC(LogicTargets::C)),
            0x02 => Some(Instructions::RLC(LogicTargets::D)),
            0x03 => Some(Instructions::RLC(LogicTargets::E)),
            0x04 => Some(Instructions::RLC(LogicTargets::H)),
            0x05 => Some(Instructions::RLC(LogicTargets::L)),
            0x06 => Some(Instructions::RLC(LogicTargets::HL)),
            0x07 => Some(Instructions::RLC(LogicTargets::A)),
            0x08 => Some(Instructions::RRC(LogicTargets::B)),
            0x09 => Some(Instructions::RRC(LogicTargets::C)),
            0x0A => Some(Instructions::RRC(LogicTargets::D)),
            0x0B => Some(Instructions::RRC(LogicTargets::E)),
            0x0C => Some(Instructions::RRC(LogicTargets::H)),
            0x0D => Some(Instructions::RRC(LogicTargets::L)),
            0x0E => Some(Instructions::RRC(LogicTargets::HL)),
            0x0F => Some(Instructions::RRC(LogicTargets::A)),
            0x10 => Some(Instructions::RL(LogicTargets::B)),
            0x11 => Some(Instructions::RL(LogicTargets::C)),
            0x12 => Some(Instructions::RL(LogicTargets::D)),
            0x13 => Some(Instructions::RL(LogicTargets::E)),
            0x14 => Some(Instructions::RL(LogicTargets::H)),
            0x15 => Some(Instructions::RL(LogicTargets::L)),
            0x16 => Some(Instructions::RL(LogicTargets::HL)),
            0x17 => Some(Instructions::RL(LogicTargets::A)),
            0x18 => Some(Instructions::RR(LogicTargets::B)),
            0x19 => Some(Instructions::RR(LogicTargets::C)),
            0x1A => Some(Instructions::RR(LogicTargets::D)),
            0x1B => Some(Instructions::RR(LogicTargets::E)),
            0x1C => Some(Instructions::RR(LogicTargets::H)),
            0x1D => Some(Instructions::RR(LogicTargets::L)),
            0x1E => Some(Instructions::RR(LogicTargets::HL)),
            0x1F => Some(Instructions::RR(LogicTargets::A)),
            0x20 => Some(Instructions::SLA(LogicTargets::B)),
            0x21 => Some(Instructions::SLA(LogicTargets::C)),
            0x22 => Some(Instructions::SLA(LogicTargets::D)),
            0x23 => Some(Instructions::SLA(LogicTargets::E)),
            0x24 => Some(Instructions::SLA(LogicTargets::H)),
            0x25 => Some(Instructions::SLA(LogicTargets::L)),
            0x26 => Some(Instructions::SLA(LogicTargets::HL)),
            0x27 => Some(Instructions::SLA(LogicTargets::A)),
            0x28 => Some(Instructions::SRA(LogicTargets::B)),
            0x29 => Some(Instructions::SRA(LogicTargets::C)),
            0x2A => Some(Instructions::SRA(LogicTargets::D)),
            0x2B => Some(Instructions::SRA(LogicTargets::E)),
            0x2C => Some(Instructions::SRA(LogicTargets::H)),
            0x2D => Some(Instructions::SRA(LogicTargets::L)),
            0x2E => Some(Instructions::SRA(LogicTargets::HL)),
            0x2F => Some(Instructions::SRA(LogicTargets::A)),
            0x30 => Some(Instructions::SWAP(LogicTargets::B)),
            0x31 => Some(Instructions::SWAP(LogicTargets::C)),
            0x32 => Some(Instructions::SWAP(LogicTargets::D)),
            0x33 => Some(Instructions::SWAP(LogicTargets::E)),
            0x34 => Some(Instructions::SWAP(LogicTargets::H)),
            0x35 => Some(Instructions::SWAP(LogicTargets::L)),
            0x36 => Some(Instructions::SWAP(LogicTargets::HL)),
            0x37 => Some(Instructions::SWAP(LogicTargets::A)),
            0x38 => Some(Instructions::SRL(LogicTargets::B)),
            0x39 => Some(Instructions::SRL(LogicTargets::C)),
            0x3A => Some(Instructions::SRL(LogicTargets::D)),
            0x3B => Some(Instructions::SRL(LogicTargets::E)),
            0x3C => Some(Instructions::SRL(LogicTargets::H)),
            0x3D => Some(Instructions::SRL(LogicTargets::L)),
            0x3E => Some(Instructions::SRL(LogicTargets::HL)),
            0x3F => Some(Instructions::SRL(LogicTargets::A))       
        }
    } else {
        let head = byte & 0xF0;
        let mut tail = byte & 0x0F;
        if tail >= 0x08 {
            tail -= 0x08;
        };
        let target: LogicTargets = match (l) {
            0x00 => LogicTargets::B,
            0x01 => LogicTargets::C,
            0x02 => LogicTargets::D,
            0x03 => LogicTargets::E,
            0x04 => LogicTargets::H,
            0x05 => LogicTargets::L,
            0x06 => LogicTargets::HL,
            0x07 => LogicTargets::A,
            _ => panic!("Invalid instruction")
        };
            
        match head {
            0x40..0x79 => Some(Instructions::BIT(tail, target)),
            0x80..0xB9 => Some(Instructions::RES(tail, target)),
            0xC0..0xF9 => Some(Instructions::SET(tail, target)),
            _ => panic!("Invalid instruction")
        }
    }
}

    fn read_byte_unprefixed(byte: u8) -> Option<Instructions> {

    }
}