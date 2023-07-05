use crate::cpu::*;

enum Opcode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP,
    JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI,
    RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA
}
use Opcode::*;

enum AddrMode {
    IMP,
    ACC,
    IMM,
    ZP,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IDX,
    IDY
}
use AddrMode::*;

pub struct Instruction(Opcode, AddrMode, u8);

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x69 => Instruction(ADC, IMM, 2),
            0x65 => Instruction(ADC, ZP, 3),
            0x75 => Instruction(ADC, ZPX, 4),
            0x6D => Instruction(ADC, ABS, 4),
            0x7D => Instruction(ADC, ABX, 4),
            0x79 => Instruction(ADC, ABY, 4),
            0x61 => Instruction(ADC, IDX, 6),
            0x71 => Instruction(ADC, IDY, 5),

            0x29 => Instruction(AND, IMM, 2),
            0x25 => Instruction(AND, ZP, 3),
            0x35 => Instruction(AND, ZPX, 4),
            0x2D => Instruction(AND, ABS, 4),
            0x3D => Instruction(AND, ABX, 4),
            0x39 => Instruction(AND, ABY, 4),
            0x21 => Instruction(AND, IDX, 6),
            0x31 => Instruction(AND, IDY, 5),

            0x0A => Instruction(ASL, ACC, 2),
            0x06 => Instruction(ASL, ZP, 5),
            0x16 => Instruction(ASL, ZPX, 6),
            0x0E => Instruction(ASL, ABS, 6),
            0x1E => Instruction(ASL, ABX, 7),

            0x90 => Instruction(BCC, REL, 2),
            0xB0 => Instruction(BCS, REL, 2),
            0xF0 => Instruction(BEQ, REL, 2),
            0x30 => Instruction(BMI, REL, 2),
            0xD0 => Instruction(BNE, REL, 2),
            0x10 => Instruction(BPL, REL, 2),
            0x50 => Instruction(BVC, REL, 2),
            0x70 => Instruction(BVS, REL, 2),

            0x24 => Instruction(BIT, ZP, 3),
            0x2C => Instruction(BIT, ABS, 4),

            0x00 => Instruction(BRK, IMP, 7),

            0x18 => Instruction(CLC, IMP, 2),
            0xD8 => Instruction(CLD, IMP, 2),
            0x58 => Instruction(CLI, IMP, 2),
            0xB8 => Instruction(CLV, IMP, 2),

            0xC5 => Instruction(CMP, ZP, 3),
            0xC9 => Instruction(CMP, IMM, 2),
            0xD5 => Instruction(CMP, ZPX, 4),
            0xCD => Instruction(CMP, ABS, 4),
            0xDD => Instruction(CMP, ABX, 4),
            0xD9 => Instruction(CMP, ABY, 4),
            0xC1 => Instruction(CMP, IDX, 6),
            0xD1 => Instruction(CMP, IDY, 5),

            0xE0 => Instruction(CPX, IMM, 2),
            0xE4 => Instruction(CPX, ZP, 3),
            0xEC => Instruction(CPX, ABS, 4),

            0xC0 => Instruction(CPY, IMM, 2),
            0xC4 => Instruction(CPY, ZP, 3),
            0xCC => Instruction(CPY, ABS, 4),

            0xC6 => Instruction(DEC, ZP, 5),
            0xD6 => Instruction(DEC, ZPX, 6),
            0xCE => Instruction(DEC, ABS, 6),
            0xDE => Instruction(DEC, ABX, 7),
            0xCA => Instruction(DEX, IMP, 2),
            0x88 => Instruction(DEY, IMP, 2),

            0x49 => Instruction(EOR, IMM, 3),
            0x45 => Instruction(EOR, ZP, 2),
            0x55 => Instruction(EOR, ZPX, 4),
            0x4D => Instruction(EOR, ABS, 4),
            0x5D => Instruction(EOR, ABX, 4),
            0x59 => Instruction(EOR, ABY, 4),
            0x41 => Instruction(EOR, IDX, 6),
            0x51 => Instruction(EOR, IDY, 5),

            0xE6 => Instruction(INC, ZP, 5),
            0xF6 => Instruction(INC, ZPX, 6),
            0xEE => Instruction(INC, ABS, 6),
            0xFE => Instruction(INC, ABX, 7),
            0xE8 => Instruction(INX, IMP, 2),
            0xC8 => Instruction(INY, IMP, 2),

            0x4C => Instruction(JMP, ABS, 3),
            0x6C => Instruction(JMP, IND, 5),
            0x20 => Instruction(JSR, ABS, 6),

            0xA9 => Instruction(LDA, IMM, 2),
            0xA5 => Instruction(LDA, ZP, 3),
            0xB5 => Instruction(LDA, ZPX, 4),
            0xAD => Instruction(LDA, ABS, 4),
            0xBD => Instruction(LDA, ABX, 4),
            0xB9 => Instruction(LDA, ABY, 4),
            0xA1 => Instruction(LDA, IDX, 6),
            0xB1 => Instruction(LDA, IDY, 5),

            0xA2 => Instruction(LDX, IMM, 2),
            0xA6 => Instruction(LDX, ZP, 3),
            0xB6 => Instruction(LDX, ZPY, 4),
            0xAE => Instruction(LDX, ABS, 4),
            0xBE => Instruction(LDX, ABY, 4),

            0xA0 => Instruction(LDY, IMM, 2),
            0xA4 => Instruction(LDY, ZP, 3),
            0xB4 => Instruction(LDY, ZPX, 4),
            0xAC => Instruction(LDY, ABS, 4),
            0xBC => Instruction(LDY, ABX, 4),
            
            0x4A => Instruction(LSR, ACC, 2),
            0x46 => Instruction(LSR, ZP, 5),
            0x56 => Instruction(LSR, ZPX, 6),
            0x4E => Instruction(LSR, ABS, 6),
            0x5E => Instruction(LSR, ABX, 7),

            0xEA => Instruction(NOP, IMP, 2),

            0x09 => Instruction(ORA, IMM, 2),
            0x05 => Instruction(ORA, ZP, 3),
            0x15 => Instruction(ORA, ZPX, 4),
            0x0D => Instruction(ORA, ABS, 4),
            0x1D => Instruction(ORA, ABX, 4),
            0x19 => Instruction(ORA, ABY, 4),
            0x01 => Instruction(ORA, IDX, 6),
            0x11 => Instruction(ORA, IDY, 5),

            0x48 => Instruction(PHA, IMP, 3),
            0x08 => Instruction(PHP, IMP, 3),
            0x68 => Instruction(PLA, IMP, 4),
            0x28 => Instruction(PLP, IMP, 4),

            0x2A => Instruction(ROL, ACC, 2),
            0x26 => Instruction(ROL, ZP, 5),
            0x36 => Instruction(ROL, ZPX, 6),
            0x2E => Instruction(ROL, ABS, 6),
            0x3E => Instruction(ROL, ABX, 7),

            0x6A => Instruction(ROR, ACC, 2),
            0x66 => Instruction(ROR, ZP, 5),
            0x76 => Instruction(ROR, ZPX, 6),
            0x6E => Instruction(ROR, ABS, 6),
            0x7E => Instruction(ROR, ABX, 7),

            0x40 => Instruction(RTI, IMP, 6),
            0x60 => Instruction(RTS, IMP, 6),

            0xE9 => Instruction(SBC, IMM, 2),
            0xE5 => Instruction(SBC, ZP, 3),
            0xF5 => Instruction(SBC, ZPX, 4),
            0xED => Instruction(SBC, ABS, 4),
            0xFD => Instruction(SBC, ABX, 4),
            0xF9 => Instruction(SBC, ABY, 4),
            0xE1 => Instruction(SBC, IDX, 6),
            0xF1 => Instruction(SBC, IDY, 5),

            0x38 => Instruction(SEC, IMP, 2),
            0xF8 => Instruction(SED, IMP, 2),
            0x78 => Instruction(SEI, IMP, 2),

            0x85 => Instruction(STA, ZP, 3),
            0x95 => Instruction(STA, ZPX, 4),
            0x8D => Instruction(STA, ABS, 4),
            0x9D => Instruction(STA, ABX, 5),
            0x99 => Instruction(STA, ABY, 5),
            0x81 => Instruction(STA, IDX, 6),
            0x91 => Instruction(STA, IDY, 6),

            0x86 => Instruction(STX, ZP, 3),
            0x96 => Instruction(STX, ZPY, 4),
            0x8E => Instruction(STX, ABS, 4),
            0x84 => Instruction(STY, ZP, 3),
            0x94 => Instruction(STY, ZPX, 4),
            0x8C => Instruction(STY, ABS, 4),

            0xAA => Instruction(TAX, IMP, 2),
            0xA8 => Instruction(TAY, IMP, 2),
            0xBA => Instruction(TSX, IMP, 2),
            0x8A => Instruction(TXA, IMP, 2),
            0x9A => Instruction(TXS, IMP, 2),
            0x98 => Instruction(TYA, IMP, 2),

            _ => panic!("Unsupported instruction: {:x}", byte)
        }
    }
}

impl Cpu {
    pub fn execute_instruction(&mut self, instr: Instruction) {
        let Instruction(opcode, mode, cycles) = instr;
        let (addr, arg) = self.get_args(&mode);

        match opcode {
            ADC => {
                let res: u16 = (self.a as u16) + (arg as u16) + (if self.get_flag(Flag::Carry) { 1 } else { 0 });

                self.set_flag(Flag::Carry, res > 0xff);
                self.set_flag(Flag::Zero, res == 0);
                self.set_flag(Flag::Overflow, (self.a as u16 ^ res) & (arg as u16 ^ res) & 0x80 == 0x80);
                self.set_flag(Flag::Negative, res & 0x80 == 0x80);
                self.a = res as u8;
            },
            AND => {
                let res = self.a & arg;

                self.set_flag(Flag::Zero, res == 0);
                self.set_flag(Flag::Negative, res & 0x80 == 0x80);
                self.a = res;
            }
            ASL => {
                let res = arg << 1;

                self.set_flag(Flag::Carry, arg & 0x80 == 0x80);
                self.set_flag(Flag::Zero, res == 0);
                self.set_flag(Flag::Negative, res & 0x80 == 0x80);

                match mode {
                    ACC => {
                        self.a = res;
                    }
                    _ => {
                        self.bus.write_byte(addr, res);
                    }
                }
            }
            BCC => {
                if !self.get_flag(Flag::Carry) {
                    self.pc.wrapping_add(arg as u16);
                }
            }
            BCS => {
                if self.get_flag(Flag::Carry) {
                    self.pc.wrapping_add(arg as u16);
                }
            }
            BEQ => {
                if self.get_flag(Flag::Zero) {
                    self.pc.wrapping_add(arg as u16);
                }
            }
            BIT => {

            }


            _ => panic!("Not implemented")
        }
    }

    pub fn get_args(&mut self, mode: &AddrMode) -> (u16, u8) {
        match mode {
            ACC => {
                (0, self.a)
            }
            IMM => {
                let arg = self.bus.read_byte(self.pc);
                self.pc += 1;

                (0, arg)
            }
            ZP => {
                let addr = self.bus.read_byte(self.pc) as u16;
                self.pc += 1;

                (0, self.bus.read_byte(addr))
            }
            ZPX => {
                let arg = self.bus.read_byte(self.pc);
                let addr = arg.wrapping_add(self.x) as u16;
                self.pc += 1;

                (0, self.bus.read_byte(addr))
            }
            ZPY => {
                let arg = self.bus.read_byte(self.pc);
                let addr = arg.wrapping_add(self.y) as u16;
                self.pc += 1;

                (0, self.bus.read_byte(addr))
            }
            REL => {
                let offset = self.bus.read_byte(self.pc);

                (0, offset)
            }
            ABS => {
                let lower_byte = self.bus.read_byte(self.pc);
                let upper_byte = self.bus.read_byte(self.pc + 1);
                let addr = ((upper_byte as u16) << 8) | lower_byte as u16;
                self.pc += 2;

                (addr, self.bus.read_byte(addr))
            }
            ABX => {
                // TODO: Deal with overflow
                let lower_byte = self.bus.read_byte(self.pc);
                let upper_byte = self.bus.read_byte(self.pc + 1);
                let addr = (((upper_byte as u16) << 8) | lower_byte as u16) + self.x as u16;

                (addr, self.bus.read_byte(addr))
            }
            ABY => {
                // TODO: Deal with overflow
                let lower_byte = self.bus.read_byte(self.pc);
                let upper_byte = self.bus.read_byte(self.pc + 1);
                let addr = (((upper_byte as u16) << 8) | lower_byte as u16) + self.y as u16;

                (addr, self.bus.read_byte(addr))
            }

            _ => {
                panic!("Not implemented")
            }
        }
    }
}
