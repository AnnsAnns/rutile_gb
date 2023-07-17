use super::{CPU, flags::FlagCondition};

mod bitshift;
mod decoder;
mod logic;
mod mem;
mod misc;
mod bitop;
mod load;
mod jumps;

#[derive(Debug)]
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
    AF,
    HL,
    SP,
    N8,
    N16,
    E8
}

#[derive(Debug)]
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
    LDHL(LogicTargets),
    LDR16(LogicTargets),
    LDR16R8(LogicTargets, LogicTargets),
    LDHN16A(LogicTargets),
    LDHCA(),
    LDHAN16(LogicTargets),
    LDHAC(),
    LDHLIA(),
    LDHLDA(),
    LDAHLD(),
    LDAHLI(),
    LDN16SP(LogicTargets),

    // Jumps and Subroutines
    CALL(LogicTargets),
    CALLC(FlagCondition, LogicTargets),
    JP(LogicTargets),
    JPC(FlagCondition, LogicTargets),
    JR(LogicTargets),
    JRC(LogicTargets, FlagCondition),
    RET(),
    RETC(FlagCondition),
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

    PREFIX(),
}

impl CPU {
    pub fn execution(&mut self, instruction: &Instructions) {
        if self.logic_execution(&instruction) {
            return;
        }
        if self.misc_execution(&instruction) {
            return;
        }
        if self.execute_bitop(&instruction) {
            return;
        }
        if self.bitshift_execution(&instruction) {
            return;
        }
        if self.execute_load(&instruction) {
            return;
        }
        if self.jump_execution(&instruction) {
            return;
        }
        panic!("Unimplemented/Invalid instruction");
    }
}
