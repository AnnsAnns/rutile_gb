pub struct Memory {
    // The Memory of the Emulator
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}