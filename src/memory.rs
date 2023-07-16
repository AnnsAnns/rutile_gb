const bootrom: &[u8; 256] = include_bytes!("../dmg_boot.bin");

pub struct Memory {
    // The Memory of the Emulator
    pub memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            memory: [0; 0xFFFF],
        };

        // Load Bootrom into Memory
        for i in 0..bootrom.len() {
            mem.memory[i] = bootrom[i];
        }

        mem
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    
    pub fn read_word(&self, address: u16) -> u16 {
        self.memory[address as usize] as u16 | ((self.memory[(address + 1) as usize] as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = (value & 0xFF) as u8;
        self.memory[(address + 1) as usize] = ((value >> 8) & 0xFF) as u8;
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}