use super::CPU;

mod bitshift;
mod decoder;
mod logic;
mod mem;
mod misc;
mod bitop;

pub enum LogicTargets {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    BC,
    DE,
    HL,
    SP,
    N8(u8),
    N16(u16),
    E8(i8),
}

pub enum Instructions {
    // LOGIC INSTRUCTIONS
    ADD(LogicTargets),
    ADDAHL(),
    ADDHLR16(LogicTargets),
    ADDHLSP(),
    ADDSPE8(LogicTargets),
    ADC(LogicTargets),
    ADCHL(),
    AND(LogicTargets),
    ANDAHL(),
    XOR(LogicTargets),
    XORHL(),
    SBC(LogicTargets),
    SBCHL(),
    OR(LogicTargets),
    ORHL(),
    SUB(LogicTargets),
    SUBHL(),
    DEC(LogicTargets),
    DECHL(),
    CP(LogicTargets),
    CPAHL(),
    INC(LogicTargets),
    INCHL(),

    // Bit Operations Instructions
    BIT(u8, LogicTargets),
    RES(u8, LogicTargets),
    SET(u8, LogicTargets),
    SWAP(LogicTargets),

    // Bit Shift Instructions
    RL(LogicTargets),
    RLC(LogicTargets),
    RLA(),
    RLCA(),
    RR(LogicTargets),
    RRC(LogicTargets),
    RRA(),
    RRCA(),
    SLA(LogicTargets),
    SRA(LogicTargets),
    SRL(LogicTargets),

    // Load Instructions
    LD(LogicTargets, LogicTargets),
    LDH(LogicTargets, LogicTargets),

    // Jumps and Subroutines
    CALL(u8),
    CALLC(u8),
    JP(u8),
    JPC(u8),
    JR(u8),
    JRC(u8),
    RET(),
    RETC(),
    RETI(),
    RST(u8),

    // Stack Op Instructions
    ADDSP(LogicTargets),
    DECSP(),
    INCSP(),
    LDSP(LogicTargets),
    POP(LogicTargets),
    PUSH(LogicTargets),

    // MISC INSTRUCTIONS
    CCF(),
    CPL(),
    DAA(),
    DI(),
    EI(),
    HALT(),
    NOP(),
    SCF(),
    STOP(),
}

impl CPU {
    pub fn execution(&mut self, instruction: Instructions) {
        if self.logic_execution(&instruction) {
            return;
        }
        if self.misc_execution(&instruction) {
            return;
        }
        panic!("Unimplemented/Invalid instruction")
    }
}
