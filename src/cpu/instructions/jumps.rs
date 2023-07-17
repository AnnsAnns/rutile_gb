use crate::cpu::{CPU, flags::FlagCondition};

use super::{LogicTargets, Instructions};

impl CPU {
    fn is_cond_true(&mut self, condition: &FlagCondition) -> bool {
        match condition {
            FlagCondition::ZZero => self.registry.f.z_zero,
            FlagCondition::NZNotZero => !self.registry.f.z_zero,
            FlagCondition::CCarry => self.registry.f.c_carry,
            FlagCondition::NCNotCarry => !self.registry.f.c_carry,
            FlagCondition::ALWAYS => true,
        }
    }

    fn jp(&mut self, target: &LogicTargets, condition: &FlagCondition) {
        if !self.is_cond_true(condition) {
            return;
        }

        match target {
            LogicTargets::HL => {
                self.registry.pc = self.registry.get_hl();
            },
            LogicTargets::N16 => {
                self.registry.pc = self.memory.read_word(self.registry.pc);
            }
            _ => panic!("Invalid JP Instruction"),
        }
    }

    fn call(&mut self, target: &LogicTargets, condition: &FlagCondition) {
        match target {
            LogicTargets::N16 => {
                if !self.is_cond_true(condition) {
                    return;
                }
                self.memory.write_word(self.registry.sp - 2, self.registry.pc + 2);
                self.registry.sp -= 2;
                self.jp(&LogicTargets::N16, &FlagCondition::ALWAYS);
            }
            _ => panic!("Invalid CALL Instruction {:?} - {:?}", target, condition),
        }
    }

    fn jr(&mut self, target: &LogicTargets, condition: &FlagCondition) {
        if !self.is_cond_true(condition) {
            return;
        }

        let value = match target {
            LogicTargets::N16 => {
                self.memory.read_word(self.registry.pc)
            }
            LogicTargets::N8 => {
                self.memory.read_byte(self.registry.pc) as u16
            }
            _ => panic!("Invalid JR Instruction"),
        };

        let new_addr = i32::from(self.registry.pc) + i32::from(value);
        self.registry.pc = (new_addr & 0xFFFF) as u16;
    }

    fn ret(&mut self, condition: &FlagCondition) {
        if !self.is_cond_true(condition) {
            return;
        }
        self.registry.pc = self.memory.read_word(self.registry.sp);
        self.registry.sp += 2;
    }

    fn rst(&mut self, target: u8) {
        self.memory.write_word(self.registry.sp - 2, self.registry.pc);
        self.registry.sp -= 2;
        self.registry.pc = target as u16;
    }

    pub fn jump_execution(&mut self, instructions: &Instructions) -> bool {
        match instructions {
            Instructions::JP(target) => self.jp(target, &FlagCondition::ALWAYS),
            Instructions::JPC(condition, target) => self.jp(target, condition),
            Instructions::CALL(target) => self.call(target, &FlagCondition::ALWAYS),
            Instructions::CALLC(cond, target) => self.call(target, cond),
            Instructions::JR(target) => self.jr(target, &FlagCondition::ALWAYS),
            Instructions::JRC(target, cond) => self.jr(target, cond),
            Instructions::RET() => self.ret(&FlagCondition::ALWAYS),
            Instructions::RETC(cond) => self.ret(cond),
            Instructions::RETI() => {
                self.ei();
                self.ret(&FlagCondition::ALWAYS)
                
            },
            Instructions::RST(target) => self.rst(*target),
            _ => return false
        }
        return true;
    }
}