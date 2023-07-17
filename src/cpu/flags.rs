// Bit	Name	Explanation
// 7	z	Zero flag
// 6	n	Subtraction flag (BCD)
// 5	h	Half Carry flag (BCD)
// 4	c	Carry flag

const ZERO_FLAG_POS: u8 = 1 << 7;
const SUBTRACTION_FLAG_POS: u8 = 1 << 6;
const HALF_CARRY_FLAG_POS: u8 = 1 << 5;
const CARRY_FLAG_POS: u8 = 1 << 4;

#[derive(Debug)]
pub enum FlagCondition {
    ZZero,
    NZNotZero,
    CCarry,
    NCNotCarry,
    ALWAYS
}

pub struct Flags {
    pub z_zero: bool,
    pub n_subtraction_bcd: bool,
    pub h_half_carry_bcd: bool,
    pub c_carry: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z_zero: false,
            n_subtraction_bcd: false,
            h_half_carry_bcd: false,
            c_carry: false,
        }
    }

    pub fn get_flags(&mut self) -> u8 {
        let mut flags: u8 = 0x00;

        if self.z_zero { flags |= ZERO_FLAG_POS; }
        if self.n_subtraction_bcd { flags |= SUBTRACTION_FLAG_POS; }
        if self.h_half_carry_bcd { flags |= HALF_CARRY_FLAG_POS; }
        if self.c_carry { flags |= CARRY_FLAG_POS; }

        flags
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.z_zero = (flags & ZERO_FLAG_POS) != 0;
        self.n_subtraction_bcd = (flags & SUBTRACTION_FLAG_POS) != 0;
        self.h_half_carry_bcd = (flags & HALF_CARRY_FLAG_POS) != 0;
        self.c_carry = (flags & CARRY_FLAG_POS) != 0;
    }
}