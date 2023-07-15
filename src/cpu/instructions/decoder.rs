use super::{Instructions, LogicTargets};

fn tail_to_logic_target(mut tail: u8) -> LogicTargets {
    if tail >= 0x08 {
        tail -= 0x08;
    };

    // I'd do this via a transmute but that'd be unsafe so lets not :P
    match (tail) {
        0x00 => LogicTargets::B,
        0x01 => LogicTargets::C,
        0x02 => LogicTargets::D,
        0x03 => LogicTargets::E,
        0x04 => LogicTargets::H,
        0x05 => LogicTargets::L,
        0x06 => LogicTargets::HL,
        0x07 => LogicTargets::A,
        _ => panic!("Invalid instruction")
    }
}

impl Instructions {
    pub fn read_byte(byte: u8, prefixed: bool) -> Option<Instructions> {
        if prefixed {
            Instructions::read_byte_prefixed(byte)
        } else {
            Instructions::read_byte_unprefixed(byte)
        }
    }

    fn read_byte_prefixed(byte: u8) -> Option<Instructions> {
        let target = tail_to_logic_target(byte & 0x07);

        match byte {
            0x00..=0x07 => Some(Instructions::RLC(target)),
            0x08..=0x0F => Some(Instructions::RRC(target)),
            0x10..=0x17 => Some(Instructions::RL(target)),
            0x18..=0x1F => Some(Instructions::RR(target)),
            0x20..=0x27 => Some(Instructions::SLA(target)),
            0x28..=0x2F => Some(Instructions::SRA(target)),
            0x30..=0x37 => Some(Instructions::SWAP(target)),
            0x38..=0x3F => Some(Instructions::SRL(target)),
            // I swear this makes sense probably. The val increases in steps of 8 
            0x40..=0x7F => Some(Instructions::BIT(byte - 0x40 / 8, target)),
            0x80..=0xBF => Some(Instructions::RES(byte - 0x80 / 8, target)),
            0xC0..=0xFF => Some(Instructions::SET(byte - 0xC0 / 8, target)),   
        }
    }

    
    fn read_byte_unprefixed(byte: u8) -> Option<Instructions> {
        let tail = tail_to_logic_target(byte & 0x07);

        match byte {
            0x00 => Some(Instructions::NOP()),
            0x10 => Some(Instructions::STOP()),
            _ => None
        }
    }
}
