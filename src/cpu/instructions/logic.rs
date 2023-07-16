use super::super::CPU;

use super::Instructions;
use super::LogicTargets;

impl CPU {
    fn inc(&mut self, target: &LogicTargets) {
        let target = match target {
            LogicTargets::A => &mut self.registry.a,
            LogicTargets::B => &mut self.registry.b,
            LogicTargets::C => &mut self.registry.c,
            LogicTargets::D => &mut self.registry.d,
            LogicTargets::E => &mut self.registry.e,
            LogicTargets::H => &mut self.registry.h,
            LogicTargets::L => &mut self.registry.l,
            _ => panic!("Unimplemented/Invalid INC target"),
        };

        *target += 1;
        self.registry.f.z_zero = *target == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = (*target & 0xF) == 0;
    }

    fn inc_16(&mut self, target: &LogicTargets) {
        let _target = match target {
            LogicTargets::BC => self.registry.set_bc(self.registry.get_bc() + 1),
            LogicTargets::DE => self.registry.set_de(self.registry.get_de() + 1),
            LogicTargets::HL => self.registry.set_hl(self.registry.get_hl() + 1),
            LogicTargets::SP => self.registry.sp -= 1,
            _ => panic!("Unimplemented/Invalid DEC target"),
        };
    }

    fn inc_hl(&mut self) {
        let hl = self.registry.get_hl();
        let target = self.memory.read_byte(hl) + 1;
        self.registry.f.z_zero = target == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = (target & 0xF) == 0;

        self.memory.write_byte(hl, target);
    }

    fn add(&mut self, value: u8, plus_carry: bool) -> u8 {
        let (add_val, overflowed) = self.registry.a.overflowing_add(value);
        // @TODO: Might be wrong way to do this?
        if plus_carry && overflowed {
            self.registry.a += 1
        };
        self.registry.a = add_val;
        self.registry.f.z_zero = add_val == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd = (self.registry.a & 0xF) + (value & 0xF) > 0xF;
        add_val
    }

    fn add_hl(&mut self, plus_carry: bool) {
        let hl = self.registry.get_hl();
        let value = self.memory.read_byte(hl);
        self.add(value, plus_carry);
    }

    fn and(&mut self, value: u8) {
        self.registry.a &= value;
        self.registry.f.z_zero = self.registry.a == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = true;
        self.registry.f.c_carry = false;
        self.registry.a;
    }

    fn sub_and_cp(&mut self, value: u8, minus_carry: bool, dont_store: bool) {
        let (sub_val, overflowed) = self.registry.a.overflowing_sub(value);
        if minus_carry && overflowed {
            self.registry.a -= 1
        };
        if !dont_store {
            self.registry.a = sub_val
        };
        self.registry.f.z_zero = sub_val == 0;
        self.registry.f.n_subtraction_bcd = true;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd = (self.registry.a & 0xF) + (value & 0xF) > 0xF;
    }

    fn or(&mut self, value: u8) -> u8 {
        self.registry.a |= value;
        self.registry.f.z_zero = self.registry.a == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
        self.registry.f.c_carry = false;
        self.registry.a
    }

    fn xor(&mut self, value: u8) -> u8 {
        self.registry.a ^= value;
        self.registry.f.z_zero = self.registry.a == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
        self.registry.f.c_carry = false;
        self.registry.a
    }

    fn dec_hl(&mut self) {
        let hl = self.registry.get_hl();
        let val = self.memory.read_byte(hl);
        let (sub_val, overflowed) = val.overflowing_sub(1);
        self.memory.write_byte(hl, sub_val);

        self.registry.f.z_zero = sub_val == 0;
        self.registry.f.n_subtraction_bcd = true;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd = (val & 0xF) == 0;
    }

    fn dec_16(&mut self, target: &LogicTargets) {
        let _target = match target {
            LogicTargets::BC => self.registry.set_bc(self.registry.get_bc() - 1),
            LogicTargets::DE => self.registry.set_de(self.registry.get_de() - 1),
            LogicTargets::HL => self.registry.set_hl(self.registry.get_hl() - 1),
            LogicTargets::SP => self.registry.sp -= 1,
            _ => panic!("Unimplemented/Invalid DEC target"),
        };
    }

    fn dec(&mut self, target: &LogicTargets) {
        let target = match target {
            LogicTargets::A => &mut self.registry.a,
            LogicTargets::B => &mut self.registry.b,
            LogicTargets::C => &mut self.registry.c,
            LogicTargets::D => &mut self.registry.d,
            LogicTargets::E => &mut self.registry.e,
            LogicTargets::H => &mut self.registry.h,
            LogicTargets::L => &mut self.registry.l,
            _ => panic!("Unimplemented/Invalid DEC target"),
        };

        *target -= 1;
        self.registry.f.z_zero = *target == 0;
        self.registry.f.n_subtraction_bcd = true;
        self.registry.f.h_half_carry_bcd = (*target & 0xF) == 0xF;
    }

    fn add_hl_r16(&mut self, target: &LogicTargets) {
        let target = match target {
            LogicTargets::BC => self.registry.get_bc(),
            LogicTargets::DE => self.registry.get_de(),
            LogicTargets::HL => self.registry.get_hl(),
            LogicTargets::SP => self.registry.sp,
            _ => panic!("Unimplemented/Invalid ADD target"),
        };

        let (add_val, overflowed) = self.registry.get_hl().overflowing_add(target);
        self.registry.set_hl(add_val);
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd =
            (self.registry.get_hl() & 0xFFF) + (target & 0xFFF) > 0xFFF;
    }

    fn add_sp_e8(&mut self, target: &LogicTargets) {
        let target = match target {
            LogicTargets::E8 => self.memory.read_byte(self.registry.sp) as i16,
            _ => panic!("Unimplemented/Invalid ADD target"),
        };

        let sp = self.registry.sp as i16;

        let (add_val, overflowed) = sp.overflowing_add(target);
        self.registry.sp = add_val as u16;
        self.registry.f.z_zero = false;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd = (self.registry.sp & 0xF) + (target as u16 & 0xF) > 0xF;
    }

    /// Returns true if the instruction is found, false if not
    pub fn logic_execution(&mut self, instruction: &Instructions) -> bool {
        match instruction {
            Instructions::ADD(target) => {
                match target {
                    LogicTargets::N8 => self.add(self.memory.read_byte(self.registry.sp), false),
                    LogicTargets::A => self.add(self.registry.a, false),
                    LogicTargets::B => self.add(self.registry.b, false),
                    LogicTargets::C => self.add(self.registry.c, false),
                    LogicTargets::D => self.add(self.registry.d, false),
                    LogicTargets::E => self.add(self.registry.e, false),
                    LogicTargets::H => self.add(self.registry.h, false),
                    LogicTargets::L => self.add(self.registry.l, false),
                    _ => panic!("Unimplemented/Invalid ADD target"),
                };
            }
            Instructions::ADDAHL() => self.add_hl(false),
            Instructions::ADDHLR16(target) => self.add_hl_r16(target),
            Instructions::ADDHLSP() => self.add_hl_r16(&LogicTargets::SP),
            Instructions::ADDSPE8(target) => self.add_sp_e8(target),
            Instructions::ADC(target) => {
                match target {
                    LogicTargets::N8 => self.add(self.memory.read_byte(self.registry.sp), true),
                    LogicTargets::A => self.add(self.registry.a, true),
                    LogicTargets::B => self.add(self.registry.b, true),
                    LogicTargets::C => self.add(self.registry.c, true),
                    LogicTargets::D => self.add(self.registry.d, true),
                    LogicTargets::E => self.add(self.registry.e, true),
                    LogicTargets::H => self.add(self.registry.h, true),
                    LogicTargets::L => self.add(self.registry.l, true),
                    _ => panic!("Unimplemented/Invalid ADC target"),
                };
            }
            Instructions::ADCHL() => self.add_hl(true),
            Instructions::AND(target) => {
                match target {
                    LogicTargets::N8 => self.and(self.memory.read_byte(self.registry.sp)),
                    LogicTargets::A => self.and(self.registry.a),
                    LogicTargets::B => self.and(self.registry.b),
                    LogicTargets::C => self.and(self.registry.c),
                    LogicTargets::D => self.and(self.registry.d),
                    LogicTargets::E => self.and(self.registry.e),
                    LogicTargets::H => self.and(self.registry.h),
                    LogicTargets::L => self.and(self.registry.l),
                    _ => panic!("Unimplemented/Invalid AND target"),
                };
            }
            Instructions::ANDAHL() => self.and(self.memory.read_byte(self.registry.get_hl())),
            Instructions::OR(target) => {
                match target {
                    LogicTargets::N8 => self.or(self.memory.read_byte(self.registry.sp)),
                    LogicTargets::A => self.or(self.registry.a),
                    LogicTargets::B => self.or(self.registry.b),
                    LogicTargets::C => self.or(self.registry.c),
                    LogicTargets::D => self.or(self.registry.d),
                    LogicTargets::E => self.or(self.registry.e),
                    LogicTargets::H => self.or(self.registry.h),
                    LogicTargets::L => self.or(self.registry.l),
                    _ => panic!("Unimplemented/Invalid OR target"),
                };
            }
            Instructions::ORHL() => {
                self.or(self.memory.read_byte(self.registry.get_hl()));
            }
            Instructions::XOR(target) => {
                match target {
                    LogicTargets::N8 => self.xor(self.memory.read_byte(self.registry.sp)),
                    LogicTargets::A => self.xor(self.registry.a),
                    LogicTargets::B => self.xor(self.registry.b),
                    LogicTargets::C => self.xor(self.registry.c),
                    LogicTargets::D => self.xor(self.registry.d),
                    LogicTargets::E => self.xor(self.registry.e),
                    LogicTargets::H => self.xor(self.registry.h),
                    LogicTargets::L => self.xor(self.registry.l),
                    _ => panic!("Unimplemented/Invalid XOR target"),
                };
            }
            Instructions::XORHL() => {
                self.xor(self.memory.read_byte(self.registry.get_hl()));
            }
            Instructions::CP(target) => {
                match target {
                    LogicTargets::N8 => self.sub_and_cp(self.memory.read_byte(self.registry.sp), false, true),
                    LogicTargets::A => self.sub_and_cp(self.registry.a, false, true),
                    LogicTargets::B => self.sub_and_cp(self.registry.b, false, true),
                    LogicTargets::C => self.sub_and_cp(self.registry.c, false, true),
                    LogicTargets::D => self.sub_and_cp(self.registry.d, false, true),
                    LogicTargets::E => self.sub_and_cp(self.registry.e, false, true),
                    LogicTargets::H => self.sub_and_cp(self.registry.h, false, true),
                    LogicTargets::L => self.sub_and_cp(self.registry.l, false, true),
                    _ => panic!("Unimplemented/Invalid CP target"),
                };
            }
            Instructions::CPAHL() => {
                self.sub_and_cp(self.memory.read_byte(self.registry.get_hl()), false, true)
            }
            Instructions::SUB(target) => {
                match target {
                    LogicTargets::N8 => self.sub_and_cp(self.memory.read_byte(self.registry.sp), false, false),
                    LogicTargets::A => self.sub_and_cp(self.registry.a, false, false),
                    LogicTargets::B => self.sub_and_cp(self.registry.b, false, false),
                    LogicTargets::C => self.sub_and_cp(self.registry.c, false, false),
                    LogicTargets::D => self.sub_and_cp(self.registry.d, false, false),
                    LogicTargets::E => self.sub_and_cp(self.registry.e, false, false),
                    LogicTargets::H => self.sub_and_cp(self.registry.h, false, false),
                    LogicTargets::L => self.sub_and_cp(self.registry.l, false, false),
                    _ => panic!("Unimplemented/Invalid SUB target"),
                };
            }
            Instructions::SUBHL() => {
                self.sub_and_cp(self.memory.read_byte(self.registry.get_hl()), false, false)
            }
            Instructions::SBC(target) => {
                match target {
                    LogicTargets::N8 => self.sub_and_cp(self.memory.read_byte(self.registry.sp), true, false),
                    LogicTargets::A => self.sub_and_cp(self.registry.a, true, false),
                    LogicTargets::B => self.sub_and_cp(self.registry.b, true, false),
                    LogicTargets::C => self.sub_and_cp(self.registry.c, true, false),
                    LogicTargets::D => self.sub_and_cp(self.registry.d, true, false),
                    LogicTargets::E => self.sub_and_cp(self.registry.e, true, false),
                    LogicTargets::H => self.sub_and_cp(self.registry.h, true, false),
                    LogicTargets::L => self.sub_and_cp(self.registry.l, true, false),
                    _ => panic!("Unimplemented/Invalid SBC target"),
                };
            }
            Instructions::SBCHL() => {
                self.sub_and_cp(self.memory.read_byte(self.registry.get_hl()), true, false)
            }
            Instructions::INC(target) => {
                match target {
                    LogicTargets::BC | LogicTargets::DE | LogicTargets::HL => self.inc_16(target),
                    _ => self.inc(target),
                };
            }
            Instructions::INCHL() => self.inc_hl(),
            Instructions::INCSP() => self.inc_16(&LogicTargets::SP),
            Instructions::DEC(target) => {
                match target {
                    LogicTargets::BC | LogicTargets::DE | LogicTargets::HL => self.dec_16(target),
                    _ => self.dec(target),
                };
            }
            Instructions::DECHL() => self.dec_hl(),
            Instructions::DECSP() => self.dec_16(&LogicTargets::SP),
            _ => return false,
        };
        return true;
    }
}
