const BOOTROM: &[u8; 256] = include_bytes!("../dmg_boot.bin");

pub struct Memory {
    // The Memory of the Emulator
    pub memory: [u8; 0xFFFFF],
    pub bootrom: [u8; 256],
    pub in_bootrom: bool,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            memory: [0; 0xFFFFF],
            bootrom: [0; 256],
            in_bootrom: true,
        };

        for i in 0..BOOTROM.len() {
            mem.bootrom[i] = BOOTROM[i];
        }

        mem
    }

    pub fn load_rom(&mut self, file: Vec<u8>) {
        for i in 0..file.len() {
            self.memory[i] = file[i];
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address < 0x100 && self.in_bootrom {
            return self.bootrom[address as usize];
        }

        self.memory[address as usize]
    }
    
    pub fn read_word(&self, address: u16) -> u16 {
        if address < 0x100 && self.in_bootrom {
            return self.bootrom[address as usize] as u16 | ((self.bootrom[(address + 1) as usize] as u16) << 8);
        }

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