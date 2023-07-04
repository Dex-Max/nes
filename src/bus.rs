pub struct Bus {
    ram: [u8; 0x800]
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; 0x800]
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ..=0x7FF => self.ram[addr as usize],
            _ => panic!("Invalid address to read")
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            ..=0x7FF => { self.ram[addr as usize] = byte },
            _ => panic!("Invalid address to write")
        }
    }
}
