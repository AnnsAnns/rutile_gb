use crate::cpu::flags::{FlagCondition};

use super::{Instructions, LogicTargets};

fn tail_to_logic_target(mut tail: u8) -> LogicTargets {
    if tail >= 0x08 {
        tail -= 0x08;
    };

    // I'd do this via a transmute but that'd be unsafe so lets not :P
    match tail {
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
            0x01 => Some(Instructions::LD(LogicTargets::BC, LogicTargets::N16)),
            0x02 => Some(Instructions::LDR16R8(LogicTargets::BC, LogicTargets::A)),
            0x03 => Some(Instructions::INC(LogicTargets::BC)),
            0x04 => Some(Instructions::INC(LogicTargets::B)),
            0x05 => Some(Instructions::DEC(LogicTargets::B)),
            0x06 => Some(Instructions::LD(LogicTargets::B, LogicTargets::N8)),
            0x07 => Some(Instructions::RLCA()),
            0x08 => Some(Instructions::LDN16SP(LogicTargets::N16)),
            0x09 => Some(Instructions::ADDHLR16(LogicTargets::BC)),
            0x0A => Some(Instructions::LD(LogicTargets::A, LogicTargets::BC)),
            0x0B => Some(Instructions::DEC(LogicTargets::BC)),
            0x0C => Some(Instructions::INC(LogicTargets::C)),
            0x0D => Some(Instructions::DEC(LogicTargets::C)),
            0x0E => Some(Instructions::LD(LogicTargets::C, LogicTargets::N8)),
            0x0F => Some(Instructions::RRCA()),
            0x10 => Some(Instructions::STOP()),
            0x11 => Some(Instructions::LD(LogicTargets::DE, LogicTargets::N16)),
            0x12 => Some(Instructions::LD(LogicTargets::DE, LogicTargets::A)),
            0x13 => Some(Instructions::INC(LogicTargets::DE)),
            0x14 => Some(Instructions::INC(LogicTargets::D)),
            0x15 => Some(Instructions::DEC(LogicTargets::D)),
            0x16 => Some(Instructions::LD(LogicTargets::D, LogicTargets::N8)),
            0x17 => Some(Instructions::RLA()),
            0x18 => Some(Instructions::JR(LogicTargets::N8)),
            0x19 => Some(Instructions::ADDHLR16(LogicTargets::DE)),
            0x1A => Some(Instructions::LD(LogicTargets::A, LogicTargets::DE)),
            0x1B => Some(Instructions::DEC(LogicTargets::DE)),
            0x1C => Some(Instructions::INC(LogicTargets::E)),
            0x1D => Some(Instructions::DEC(LogicTargets::E)),
            0x1E => Some(Instructions::LD(LogicTargets::E, LogicTargets::N8)),
            0x1F => Some(Instructions::RRA()),
            0x20 => Some(Instructions::JR(LogicTargets::N8)),
            0x21 => Some(Instructions::LD(LogicTargets::HL, LogicTargets::N16)),
            // 0x22 => Some(Instructions::LDI(LogicTargets::HL, LogicTargets::A)),
            0x23 => Some(Instructions::INC(LogicTargets::HL)),
            0x24 => Some(Instructions::INC(LogicTargets::H)),
            0x25 => Some(Instructions::DEC(LogicTargets::H)),
            0x26 => Some(Instructions::LD(LogicTargets::H, LogicTargets::N8)),
            0x27 => Some(Instructions::DAA()),
            0x28 => Some(Instructions::JR(LogicTargets::N8)),
            0x29 => Some(Instructions::ADDHLR16(LogicTargets::HL)),
            // 0x2A => Some(Instructions::LDI(LogicTargets::A, LogicTargets::HL)),
            0x2B => Some(Instructions::DEC(LogicTargets::HL)),
            0x2C => Some(Instructions::INC(LogicTargets::L)),
            0x2D => Some(Instructions::DEC(LogicTargets::L)),
            0x2E => Some(Instructions::LD(LogicTargets::L, LogicTargets::N8)),
            0x2F => Some(Instructions::CPL()),
            0x30 => Some(Instructions::JR(LogicTargets::N8)),
            0x31 => Some(Instructions::LD(LogicTargets::SP, LogicTargets::N16)),
            0x32 => Some(Instructions::LDAHLD()),
            0x33 => Some(Instructions::INC(LogicTargets::SP)),
            0x34 => Some(Instructions::INCHL()),
            0x35 => Some(Instructions::DECHL()),
            0x36 => Some(Instructions::LD(LogicTargets::HL, LogicTargets::N8)),
            0x37 => Some(Instructions::SCF()),
            0x38 => Some(Instructions::JR(LogicTargets::N8)),
            0x39 => Some(Instructions::ADDHLR16(LogicTargets::SP)),
            0x3A => Some(Instructions::LDAHLI()),
            0x3B => Some(Instructions::DEC(LogicTargets::SP)),
            0x3C => Some(Instructions::INC(LogicTargets::A)),
            0x3D => Some(Instructions::DEC(LogicTargets::A)),
            0x3E => Some(Instructions::LD(LogicTargets::A, LogicTargets::N8)),
            0x3F => Some(Instructions::CCF()),
            0x40..=0x47 => Some(Instructions::LD(LogicTargets::B, tail)),
            0x48..=0x4F => Some(Instructions::LD(LogicTargets::C, tail)),
            0x50..=0x57 => Some(Instructions::LD(LogicTargets::D, tail)),
            0x58..=0x5F => Some(Instructions::LD(LogicTargets::E, tail)),
            0x60..=0x67 => Some(Instructions::LD(LogicTargets::H, tail)),
            0x68..=0x6F => Some(Instructions::LD(LogicTargets::L, tail)),
            0x70..=0x75 | 0x77 => Some(Instructions::LDR16R8(LogicTargets::HL, tail)),
            0x76 => Some(Instructions::HALT()),
            0x78..=0x7F => Some(Instructions::LD(LogicTargets::A, tail)),
            0x80..=0x87 => Some(Instructions::ADD(tail)),
            0x88..=0x8F => Some(Instructions::ADC(tail)),
            0x90..=0x97 => Some(Instructions::SUB(tail)),
            0x98..=0x9F => Some(Instructions::SBC(tail)),
            0xA0..=0xA7 => Some(Instructions::AND(tail)),
            0xA8..=0xAF => Some(Instructions::XOR(tail)),
            0xB0..=0xB7 => Some(Instructions::OR(tail)),
            0xB8..=0xBF => Some(Instructions::CP(tail)),
            0xC0 => Some(Instructions::RETC(FlagCondition::NZNotZero)),
            0xC1 => Some(Instructions::POP(LogicTargets::BC)),
            0xC2 => Some(Instructions::JPC(FlagCondition::NZNotZero, LogicTargets::N16)),
            0xC3 => Some(Instructions::JP(LogicTargets::N16)),
            0xC4 => Some(Instructions::CALLC(FlagCondition::NZNotZero, LogicTargets::N16)),
            0xC5 => Some(Instructions::PUSH(LogicTargets::BC)),
            0xC6 => Some(Instructions::ADD(LogicTargets::N8)),
            0xC7 => Some(Instructions::RST(0x00)),
            0xC8 => Some(Instructions::RETC(FlagCondition::ZZero)),
            0xC9 => Some(Instructions::RET()),
            0xCA => Some(Instructions::JPC(FlagCondition::ZZero, LogicTargets::N16)),
            0xCB => Some(Instructions::PREFIX()),
            0xCC => Some(Instructions::CALLC(FlagCondition::ZZero, LogicTargets::N16)),
            0xCD => Some(Instructions::CALL(LogicTargets::N16)),
            0xCE => Some(Instructions::ADC(LogicTargets::N8)),
            0xCF => Some(Instructions::RST(0x08)),
            0xD0 => Some(Instructions::RETC(FlagCondition::NCNotCarry)),
            0xD1 => Some(Instructions::POP(LogicTargets::DE)),
            0xD2 => Some(Instructions::JPC(FlagCondition::NCNotCarry, LogicTargets::N16)),
            0xD4 => Some(Instructions::CALLC(FlagCondition::NCNotCarry, LogicTargets::N16)),
            0xD5 => Some(Instructions::PUSH(LogicTargets::DE)),
            0xD6 => Some(Instructions::SUB(LogicTargets::N8)),
            0xD7 => Some(Instructions::RST(0x10)),
            0xD8 => Some(Instructions::RETC(FlagCondition::CCarry)),
            0xD9 => Some(Instructions::RETI()),
            0xDA => Some(Instructions::JPC(FlagCondition::CCarry, LogicTargets::N16)),
            0xDC => Some(Instructions::CALLC(FlagCondition::CCarry, LogicTargets::N16)),
            0xDE => Some(Instructions::SBC(LogicTargets::N8)),
            0xDF => Some(Instructions::RST(0x18)),
            0xE0 => Some(Instructions::LD(LogicTargets::N8, LogicTargets::A)), // Might be wrong!
            0xE1 => Some(Instructions::POP(LogicTargets::HL)),
            0xE2 => Some(Instructions::LD(LogicTargets::C, LogicTargets::A)),
            0xE5 => Some(Instructions::PUSH(LogicTargets::HL)),
            0xE6 => Some(Instructions::AND(LogicTargets::N8)),
            0xE7 => Some(Instructions::RST(0x20)),
            0xE8 => Some(Instructions::ADDSP(LogicTargets::E8)),
            0xE9 => Some(Instructions::JP(LogicTargets::HL)),
            0xEA => Some(Instructions::LD(LogicTargets::N16, LogicTargets::A)),
            0xEE => Some(Instructions::XOR(LogicTargets::N8)),
            0xEF => Some(Instructions::RST(0x28)),
            0xF0 => Some(Instructions::LD(LogicTargets::A, LogicTargets::N8)), // Might be wrong!
            0xF1 => Some(Instructions::POP(LogicTargets::AF)),
            0xF2 => Some(Instructions::LD(LogicTargets::A, LogicTargets::C)),
            0xF3 => Some(Instructions::DI()),
            0xF5 => Some(Instructions::PUSH(LogicTargets::AF)),
            0xF6 => Some(Instructions::OR(LogicTargets::N8)),
            0xF7 => Some(Instructions::RST(0x30)),
            0xF8 => None, // Not correct
            0xF9 => Some(Instructions::LD(LogicTargets::SP, LogicTargets::HL)),
            0xFA => Some(Instructions::LD(LogicTargets::A, LogicTargets::N16)),
            0xFB => Some(Instructions::EI()),
            0xFE => Some(Instructions::CP(LogicTargets::N8)),
            0xFF => Some(Instructions::RST(0x38)),
            _ => None
        }
    }
}
