use super::{super::CPU, Instructions};

impl CPU {
    fn ccf(&mut self) {
        self.registry.f.c_carry = !self.registry.f.c_carry;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
    }

    fn cpl(&mut self) {
        self.registry.a = !self.registry.a;
        self.registry.f.n_subtraction_bcd = true;
        self.registry.f.h_half_carry_bcd = true;
    }
    
    fn daa(&mut self) {
        let mut a = self.registry.a;
        let mut adjust = 0;
        if self.registry.f.h_half_carry_bcd || (!self.registry.f.n_subtraction_bcd && (a & 0xF) > 9) {
            adjust |= 0x06;
        }
        if self.registry.f.c_carry || (!self.registry.f.n_subtraction_bcd && a > 0x99) {
            adjust |= 0x60;
            self.registry.f.c_carry = true;
        }
        a = if self.registry.f.n_subtraction_bcd {a.wrapping_sub(adjust)} else {a.wrapping_add(adjust)};
        self.registry.f.z_zero = a == 0;
        self.registry.f.h_half_carry_bcd = false;
        self.registry.a = a;
    }

    fn di(&mut self) {
        self.registry.interrupts_enabled = false;
    }

    pub fn ei(&mut self) {
        self.registry.interrupts_enabled = true;
    }

    fn halt(&mut self) {
        self.registry.halted = true;
    }

    fn scf(&mut self) {
        self.registry.f.c_carry = true;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
    }

    fn stop(&mut self) {
        self.registry.verylowpowermode = true;
    }

    pub fn misc_execution(&mut self, instruction: &Instructions) -> bool {
        match instruction {
            Instructions::CCF() => self.ccf(),
            Instructions::CPL() => self.cpl(),
            Instructions::DAA() => self.daa(),
            Instructions::DI() => self.di(),
            Instructions::EI() => self.ei(),
            Instructions::HALT() => self.halt(),
            Instructions::NOP() => (), // Do Nothing
            Instructions::SCF() => self.scf(),
            Instructions::STOP() => self.stop(),
            _ => return false,
        };
        return true;
    } 
}