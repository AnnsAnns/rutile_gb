use super::flags::Flags;

pub struct CPURegistry {
    // 16-bit	Hi	Lo	Name/Function
    // AF	A	-	Accumulator & Flags 
    pub a: u8,
    pub f: Flags,
    // BC	B	C	BC
    pub b: u8,
    pub c: u8,
    // DE	D	E	DE
    pub d: u8,
    pub e: u8,
    // HL	H	L	HL
    pub h: u8,
    pub l: u8,
    // SP	-	-	Stack Pointer
    pub sp: u16,
    // PC	-	-	Program Counter/Pointer 
    pub pc: u16,
}

impl CPURegistry { 
    /// Get Accumulator & Flags 
    pub fn get_af(&mut self) -> u16 {
        ((self.a as u16) << 8) | (self.f.get_flags() as u16)
    }

    /// Set Accumulator & Flags
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f.set_flags((value & 0x00FF) as u8);
    }

    /// Get BC
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    /// Set BC
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    /// Get DE
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    /// Set DE
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    /// Get HL
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    /// Set HL
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    /// Get Stack Pointer
    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    /// Set Stack Pointer
    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }
}