mod registry;
mod flags;
mod instructions;

use crate::memory::Memory;

use self::instructions::Instructions;

pub struct CPU {
    // The Main Engine of the Emulator
    registry: registry::CPURegistry,
    memory: Memory
}

impl CPU {
    fn step(&mut self) {
        let mut opcode = self.memory.read_byte(self.registry.pc);
        let prefixed = opcode == 0xCB;
        if prefixed {
          opcode = self.memory.read_byte(self.registry.pc + 1);
        }
        self.execution(Instructions::read_byte(opcode, prefixed).unwrap_or(Instructions::NOP()));
    }
}