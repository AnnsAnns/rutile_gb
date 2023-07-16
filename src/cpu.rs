mod registry;
mod flags;
pub(crate) mod instructions;

use crate::memory::Memory;

use self::instructions::Instructions;

pub struct CPU {
    // The Main Engine of the Emulator
    pub registry: registry::CPURegistry,
    pub memory: Memory,
    pub last_instruction: Instructions
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registry: registry::CPURegistry::new(),
            memory: Memory::new(),
            last_instruction: Instructions::NOP()
        }
    }

    pub fn step(&mut self) {
        let mut opcode = self.memory.read_byte(self.registry.pc);
        let prefixed = opcode == 0xCB;
        if prefixed {
          opcode = self.memory.read_byte(self.registry.pc + 1);
          self.registry.pc += 1;
        }
        let previous_pc = self.registry.pc;
        let instruction = Instructions::read_byte(opcode, prefixed).unwrap_or(Instructions::NOP());
        self.execution(&instruction);
        
        if self.registry.pc == previous_pc {
            self.registry.pc += 1;
        }
        self.last_instruction = instruction;
    }
}