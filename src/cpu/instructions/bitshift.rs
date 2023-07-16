use crate::cpu::CPU;

use super::{LogicTargets, Instructions};

impl CPU {
    fn rotate(&mut self, target: &LogicTargets, through_carry: bool, left: bool) {
        let carry = self.registry.f.c_carry;

        let register: &mut u8 = match target {
            LogicTargets::A => &mut self.registry.a,
            LogicTargets::B => &mut self.registry.b,
            LogicTargets::C => &mut self.registry.c,
            LogicTargets::D => &mut self.registry.d,
            LogicTargets::E => &mut self.registry.e,
            LogicTargets::H => &mut self.registry.h,
            LogicTargets::L => &mut self.registry.l,
            _ => panic!("Invalid RL Instruction"),
        };

        let new_carry = *register & 0x80 == 0x80;
        if left {
            if through_carry {
                *register = (*register << 1) | carry as u8;
            } else {
                *register = (*register << 1) | new_carry as u8;
            }
        } else {
            if through_carry {
                *register = (*register >> 1) | (carry as u8) << 7;
            } else {
                *register = (*register >> 1) | (new_carry as u8) << 7;
            }
        }
        self.registry.f.z_zero = *register == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
        self.registry.f.c_carry = new_carry;
    }

    fn rotate_hl(&mut self, through_carry: bool, left: bool) {
        let carry = self.registry.f.c_carry;

        let mut target = self.memory.read_byte(self.registry.get_hl());

        let new_carry = target & 0x80 == 0x80;

        if(left) {
            if through_carry {
                target = (target << 1) | carry as u8;
            } else {
                target = (target << 1) | new_carry as u8;
            }
        } else {
            if through_carry {
                target = (target >> 1) | (carry as u8) << 7;
            } else {
                target = (target >> 1) | (new_carry as u8) << 7;
            }
        }
        target = (target << 1) | carry as u8;
        self.registry.f.z_zero = target == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
        self.registry.f.c_carry = new_carry;

        self.memory.write_byte(self.registry.get_hl(), target);
    }

    pub fn bitshift_execution(&mut self, instructions: &Instructions) -> bool {
        match instructions {
            Instructions::RL(target) => {
                match target {
                    LogicTargets::HL => self.rotate_hl(true, true),
                    _ => self.rotate(&target, true, true),
                }
            },
            Instructions::RLA() => self.rotate(&LogicTargets::A, true, true),
            Instructions::RLC(target) => {
                match target {
                    LogicTargets::HL => self.rotate_hl(false, true),
                    _ => self.rotate(&target, false, true),
                }
            },
            Instructions::RLCA() => self.rotate(&LogicTargets::A, false, true),
            Instructions::RR(target) => {
                match target {
                    LogicTargets::HL => self.rotate_hl(true, false),
                    _ => self.rotate(&target, true, false),
                }
            },
            Instructions::RRA() => self.rotate(&LogicTargets::A, true, false),
            Instructions::RRC(target) => {
                match target {
                    LogicTargets::HL => self.rotate_hl(false, false),
                    _ => self.rotate(&target, false, false),
                }
            },
            Instructions::RRCA() => self.rotate(&LogicTargets::A, false, false),
            _ => return false,
        }
        return true;
    }
}