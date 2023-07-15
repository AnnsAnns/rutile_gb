use super::super::CPU;

pub enum Instructions {
    ADD(LogicTargets),
    ADC(LogicTargets),
    AND(LogicTargets),
    XOR(LogicTargets),
    SBC(LogicTargets),
    OR(LogicTargets),
    SUB(LogicTargets),
    CP(LogicTargets),
}

pub enum LogicTargets {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl CPU {
    fn add(&mut self, value: u8, plus_carry: bool) -> u8 {
        let (add_val, overflowed) = self.registry.a.overflowing_add(value);
        // @TODO: Might be wrong way to do this?
        if plus_carry && overflowed {self.registry.a += 1};
        self.registry.a = add_val;
        self.registry.f.z_zero = add_val == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.c_carry = overflowed;
        self.registry.f.h_half_carry_bcd = (self.registry.a & 0xF) + (value & 0xF) > 0xF;
        add_val
    }

    fn and(&mut self, value: u8) -> u8 {
        self.registry.a &= value;
        self.registry.f.z_zero = self.registry.a == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = true;
        self.registry.f.c_carry = false;
        self.registry.a
    }
    
    fn sub_and_cp(&mut self, value: u8, minus_carry: bool, dont_store: bool) {
        let (sub_val, overflowed) = self.registry.a.overflowing_sub(value);
        if minus_carry && overflowed {self.registry.a -= 1};
        if !dont_store {self.registry.a = sub_val};
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

    fn logic_execution(&mut self, instruction: Instructions) {
        match instruction {
            Instructions::ADD(target) => {
                match target {
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
            Instructions::ADC(target) => {
                match target {
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
            Instructions::AND(target) => {
                match target {
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
            Instructions::OR(target) => {
                match target {
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
            Instructions::XOR(target) => {
                match target {
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
            Instructions::CP(target) => {
                match target {
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
            Instructions::SUB(target) => {
                match target {
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
            Instructions::SBC(target) => {
                match target {
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
            
            _ => panic!("Unimplemented/Invalid instruction"),
        }
    }
}