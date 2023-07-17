use crate::cpu::CPU;

use super::{Instructions, LogicTargets};


impl CPU {
    pub fn target_to_value_r8(&mut self, target: &LogicTargets) -> u8 {
        match target {
            LogicTargets::A => self.registry.a,
            LogicTargets::B => self.registry.b,
            LogicTargets::C => self.registry.c,
            LogicTargets::D => self.registry.d,
            LogicTargets::E => self.registry.e,
            LogicTargets::H => self.registry.h,
            LogicTargets::L => self.registry.l,
            LogicTargets::N8 => self.memory.read_byte(self.registry.pc),
            LogicTargets::HL |
            LogicTargets::AF |
            LogicTargets::BC |
            LogicTargets::DE => {
                let val = self.target_to_value_r16(target);
                self.memory.read_byte(val)
            },

            _ => panic!("Invalid target_to_value_r8 {:#?}", target),
        }
    }

    pub fn target_to_value_r16(&mut self, target: &LogicTargets) -> u16 {
        match target {
            LogicTargets::BC => self.registry.get_bc(),
            LogicTargets::DE => self.registry.get_de(),
            LogicTargets::HL => self.registry.get_hl(),
            LogicTargets::AF => self.registry.get_af(),
            LogicTargets::N16 => self.memory.read_word(self.registry.pc),
            LogicTargets::SP => self.registry.sp,
            _ => panic!("Invalid target_to_value_r16 {:#?}", target),
        }
    }

    fn ld_r8(&mut self, target: &LogicTargets, value: &LogicTargets) {
        match target {
            LogicTargets::A => self.registry.a = self.target_to_value_r8(value),
            LogicTargets::B => self.registry.b = self.target_to_value_r8(value),
            LogicTargets::C => self.registry.c = self.target_to_value_r8(value),
            LogicTargets::D => self.registry.d = self.target_to_value_r8(value),
            LogicTargets::E => self.registry.e = self.target_to_value_r8(value),
            LogicTargets::H => self.registry.h = self.target_to_value_r8(value),
            LogicTargets::L => self.registry.l = self.target_to_value_r8(value),
            _ => panic!("Invalid SET Instruction"),
        }
    }

    fn ld_r16(&mut self, target: &LogicTargets, value: &LogicTargets) {
        let value = self.target_to_value_r16(value);

        match target {
            LogicTargets::BC => self.registry.set_bc(value),
            LogicTargets::DE => self.registry.set_bc(value),
            LogicTargets::HL => self.registry.set_hl(value),
            LogicTargets::AF => self.registry.set_af(value),
            LogicTargets::SP => self.registry.sp = value,
            _ => panic!("Invalid SET Instruction"),
        }
    }

    fn ld_mem_r8(&mut self, value: &LogicTargets, target: &LogicTargets) {
        let value = self.target_to_value_r8(value);
        let address = match target {
            LogicTargets::HL => self.registry.get_hl(),
            LogicTargets::N16 => self.memory.read_word(self.registry.pc),
            _ => panic!("Invalid LD HL R8 Instruction"),
        };
        self.memory.write_byte(address, value);
    }

    /// Store value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
    fn ldh_r16_mem(&mut self, target: &LogicTargets, use_c: bool) {
        let address = match target {
            LogicTargets::N16 => self.memory.read_word(self.registry.pc),
            _ => panic!("Invalid LDH R16 MEM Instruction"),
        };

        if address < 0xFF00 {
            return;
        } else if use_c {
            let c = self.registry.c;
            self.memory.write_byte(address + c as u16, self.registry.a);
        } else { 
            self.memory.write_byte(address, self.registry.a);
        }
    }

    fn ldhc_mem(&mut self, use_c: bool) {
        let address = 0xFF00;

        if use_c {
            let c = self.registry.c;
            self.memory.write_byte(address + c as u16, self.registry.a);
        } else { 
            self.memory.write_byte(address, self.registry.a);
        }
    }

    fn ldh_a_n16(&mut self, target: &LogicTargets) {
        let address = match target {
            LogicTargets::N16 => self.memory.read_word(self.registry.pc),
            _ => panic!("Invalid LDH A N16 Instruction"),
        };

        if address < 0xFF00 {
            return;
        } else {
            self.registry.a = self.memory.read_byte(address);
        }
    }

    fn ldh_a_c(&mut self) {
        let c = self.registry.c;
        self.registry.a = self.memory.read_byte(0xFF00 + c as u16);
    }

    pub fn execute_load(&mut self, instruction: &Instructions) -> bool{
        match instruction {
            Instructions::LD(target, value) => {
                match target {
                    LogicTargets::A | LogicTargets::B | LogicTargets::C | LogicTargets::D | LogicTargets::E | LogicTargets::H | LogicTargets::L => {
                        self.ld_r8(target, value);
                    },
                    LogicTargets::BC | LogicTargets::DE | LogicTargets::HL | LogicTargets::AF | LogicTargets::SP => {
                        self.ld_r16(target, value);
                    },
                    _ => panic!("Invalid SET Instruction {:?} {:?}", target, value),
                }
            },
            Instructions::LDHL(value) => self.ld_mem_r8(&LogicTargets::HL, value),
            Instructions::LDR16(target) => self.ld_mem_r8(&LogicTargets::A, target),
            Instructions::LDHN16A(target) => self.ldh_r16_mem(target, false),
            Instructions::LDHCA() => self.ldhc_mem( true),
            Instructions::LDHAN16(target) => self.ldh_a_n16(target),
            Instructions::LDHAC() => self.ldh_a_c(),
            Instructions::LDHLIA() => {
                self.ld_mem_r8(&LogicTargets::HL, &LogicTargets::A);
                self.registry.set_hl(self.registry.get_hl() + 1);
            },
            Instructions::LDHLDA() => {
                // TODO: Check if this is correct
                self.ld_mem_r8(&LogicTargets::HL, &LogicTargets::A);
                self.registry.set_hl(self.registry.get_hl() - 1);
            },
            Instructions::LDAHLD() => {
                self.registry.a = self.memory.read_byte(self.registry.get_hl());
                self.registry.set_hl(self.registry.get_hl() - 1);
            },
            Instructions::LDAHLI() => {
                self.registry.a = self.memory.read_byte(self.registry.get_hl());
                self.registry.set_hl(self.registry.get_hl() + 1);
            },
            Instructions::LDN16SP(target) => {
                let target = match target {
                    LogicTargets::N16 => self.memory.read_word(self.registry.pc),
                    _ => panic!("Invalid LD N16 SP Instruction"),
                };
                self.memory.write_byte(target, (self.registry.sp & 0xFF) as u8);
                self.memory.write_byte(target+1, (self.registry.sp >> 8) as u8);
            }
            _ => return false
        }
        return true;
    }
}