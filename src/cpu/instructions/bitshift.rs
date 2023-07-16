use crate::cpu::CPU;

use super::LogicTargets;

impl CPU {
    fn rl(&mut self, val: &u8) {
        let carry = self.registry.f.c_carry as u8;
        let new_carry = *val & 0x80 == 0x80;
        self.registry.f.c_carry = new_carry;
        let new_val = (*val << 1) | carry;
        self.registry.f.z_zero = new_val == 0;
        self.registry.f.n_subtraction_bcd = false;
        self.registry.f.h_half_carry_bcd = false;
    }
}