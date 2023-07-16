use crate::cpu::CPU;

use super::{Instructions, LogicTargets};

impl CPU {
    fn bit(&mut self, test_bit: &u8, target: u8) {
        let bit = 1 << test_bit;
        self.registry.f.z_zero = (target & bit) == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = true;
    }

    fn res(&mut self, bit: &u8, target: &LogicTargets) {
        match target {
            LogicTargets::A => self.registry.a &= !(1 << bit),
            LogicTargets::B => self.registry.b &= !(1 << bit),
            LogicTargets::C => self.registry.c &= !(1 << bit),
            LogicTargets::D => self.registry.d &= !(1 << bit),
            LogicTargets::E => self.registry.e &= !(1 << bit),
            LogicTargets::H => self.registry.h &= !(1 << bit),
            LogicTargets::L => self.registry.l &= !(1 << bit),
            LogicTargets::HL => {
                let address = self.registry.get_hl();
                let value = self.memory.read_byte(address) & !(1 << bit);
                self.memory.write_byte(address, value);
            }
            _ => panic!("Invalid RES Instruction"),
        }
    }

    fn set(&mut self, bit: &u8, target: &LogicTargets) {
        match target {
            LogicTargets::A => self.registry.a |= 1 << bit,
            LogicTargets::B => self.registry.b |= 1 << bit,
            LogicTargets::C => self.registry.c |= 1 << bit,
            LogicTargets::D => self.registry.d |= 1 << bit,
            LogicTargets::E => self.registry.e |= 1 << bit,
            LogicTargets::H => self.registry.h |= 1 << bit,
            LogicTargets::L => self.registry.l |= 1 << bit,
            LogicTargets::HL => {
                let address = self.registry.get_hl();
                let value = self.memory.read_byte(address) | 1 << bit;
                self.memory.write_byte(address, value);
            }
            _ => panic!("Invalid SET Instruction"),
        }
    }

    fn swap(&mut self, target: &LogicTargets) {
        match target {
            LogicTargets::A => self.registry.a = (self.registry.a << 4) | (self.registry.a >> 4),
            LogicTargets::B => self.registry.b = (self.registry.b << 4) | (self.registry.b >> 4),
            LogicTargets::C => self.registry.c = (self.registry.c << 4) | (self.registry.c >> 4),
            LogicTargets::D => self.registry.d = (self.registry.d << 4) | (self.registry.d >> 4),
            LogicTargets::E => self.registry.e = (self.registry.e << 4) | (self.registry.e >> 4),
            LogicTargets::H => self.registry.h = (self.registry.h << 4) | (self.registry.h >> 4),
            LogicTargets::L => self.registry.l = (self.registry.l << 4) | (self.registry.l >> 4),
            LogicTargets::HL => {
                let address = self.registry.get_hl();
                let value = self.memory.read_byte(address);
                self.memory.write_byte(address, (value << 4) | (value >> 4));
            }
            _ => panic!("Invalid SWAP Instruction"),
        }
    }

    fn execute_bitop(&mut self, instruction: &Instructions) -> bool {
        match instruction {
            Instructions::BIT(test_bit, target) => match target {
                LogicTargets::A => self.bit(test_bit, self.registry.a),
                LogicTargets::B => self.bit(test_bit, self.registry.b),
                LogicTargets::C => self.bit(test_bit, self.registry.c),
                LogicTargets::D => self.bit(test_bit, self.registry.d),
                LogicTargets::E => self.bit(test_bit, self.registry.e),
                LogicTargets::H => self.bit(test_bit, self.registry.h),
                LogicTargets::L => self.bit(test_bit, self.registry.l),
                LogicTargets::HL => {
                    self.bit(test_bit, self.memory.read_byte(self.registry.get_hl()))
                }
                _ => panic!("Invalid BIT Instruction"),
            },
            Instructions::RES(bit, target) => self.res(bit, target),
            Instructions::SET(bit, target) => self.set(bit, target),
            Instructions::SWAP(target) => self.swap(target),
            _ => return false
        }
        return true;
    }
}
