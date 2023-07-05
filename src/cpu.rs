use crate::instruction::Instruction;
use crate::bus::Bus;

pub enum Flag {
    Carry = 1,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2, 
    Decimal = 1 << 3,
    Overflow = 1 << 6,
    Negative = 1 << 7
}

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    sp: u8,
    p: u8,
    pub bus: Bus
}


impl Cpu {
    pub fn new() -> Cpu {
        let bus = Bus::new();

        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xff,
            p: 0,
            bus
        }
    }

    pub fn push_stack(&mut self, value: u8) {
        self.bus.write_byte(0x0100 + self.sp as u16, value);
        self.sp.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.sp.wrapping_add(1);
        return self.bus.read_byte(0x0100 + self.sp as u16);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.p & (flag as u8) != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        self.p = if value { self.p | flag as u8 } else { self.p & !(flag as u8) }
    }
}
