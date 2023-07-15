use super::CPU;

mod logic;
mod misc;

pub enum LogicTargets {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Instructions {
    // LOGIC INSTRUCTIONS
    ADD(LogicTargets),
    ADC(LogicTargets),
    AND(LogicTargets),
    XOR(LogicTargets),
    SBC(LogicTargets),
    OR(LogicTargets),
    SUB(LogicTargets),
    DEC(LogicTargets),
    CP(LogicTargets),
    INC(LogicTargets),

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
    fn execution(&mut self, instruction: Instructions) {
        if self.logic_execution(&instruction) {return;}
        if self.misc_execution(&instruction) {return;}
        panic!("Unimplemented/Invalid instruction")
    }
}