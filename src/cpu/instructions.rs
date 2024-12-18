use crate::cpu::addressing_modes::*;
use crate::cpu::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub addressing_mode: AddressingModes,
    pub cycles: u8,
}

pub struct InstructionResult {
    pub executed_cycles: u8,
}

impl Instruction {
    pub fn execute(&self, cpu: &mut CPU) -> InstructionResult {
        // println!("instruction {}, PC {}", self.name, cpu.program_counter);
        match self.name {
            "AAC" => aac(self, cpu),
            "ADC" => adc(self, cpu),
            "AND" => and(self, cpu),
            "ASL" => asl(self, cpu),
            "BCC" => bcc(self, cpu),
            "BCS" => bcs(self, cpu),
            "BEQ" => beq(self, cpu),
            "BIT" => bit(self, cpu),
            "BMI" => bmi(self, cpu),
            "BNE" => bne(self, cpu),
            "BPL" => bpl(self, cpu),
            "BVC" => bvc(self, cpu),
            "BVS" => bvs(self, cpu),
            "CLC" => clc(self, cpu),
            "CLD" => cld(self, cpu),
            "CLI" => cli(self, cpu),
            "CLV" => clv(self, cpu),
            "CMP" => cmp(self, cpu),
            "CPX" => cpx(self, cpu),
            "CPY" => cpy(self, cpu),
            "DEC" => dec(self, cpu),
            "DEX" => dex(self, cpu),
            "DEY" => dey(self, cpu),
            "DOP" => dop(self, cpu),
            "EOR" => eor(self, cpu),
            "BRK" => brk(self, cpu),
            "LDA" => lda(self, cpu),
            "LDX" => ldx(self, cpu),
            "LDY" => ldy(self, cpu),
            "LSR" => lsr(self, cpu),
            "NOP" => nop(self, cpu),
            "JMP" => jmp(self, cpu),
            "JSR" => jsr(self, cpu),
            "ORA" => ora(self, cpu),
            "PHA" => pha(self, cpu),
            "PHP" => php(self, cpu),
            "PLA" => pla(self, cpu),
            "PLP" => plp(self, cpu),
            "INC" => inc(self, cpu),
            "INX" => inx(self, cpu),
            "INY" => iny(self, cpu),
            "RLA" => rla(self, cpu),
            "ROL" => rol(self, cpu),
            "ROR" => ror(self, cpu),
            "RTI" => rti(self, cpu),
            "RTS" => rts(self, cpu),
            "SBC" => sbc(self, cpu),
            "SEC" => sec(self, cpu),
            "SED" => sed(self, cpu),
            "SLO" => slo(self, cpu),
            "SEI" => sei(self, cpu),
            "STA" => sta(self, cpu),
            "STX" => stx(self, cpu),
            "STY" => sty(self, cpu),
            "TAX" => tax(self, cpu),
            "TAY" => tay(self, cpu),
            "TOP" => top(self, cpu),
            "TSX" => tsx(self, cpu),
            "TXA" => txa(self, cpu),
            "TXS" => txs(self, cpu),
            "TYA" => tya(self, cpu),
            _ => panic!(),
        }
    }
}

#[rustfmt::skip]
pub static INSTRUCTIONS: Lazy<HashMap<u8, Instruction>> = Lazy::new(|| {
    vec![
        Instruction {opcode: 0x69, name: "ADC", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x65, name: "ADC", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x75, name: "ADC", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x6D, name: "ADC", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x7D, name: "ADC", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x79, name: "ADC", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0x61, name: "ADC", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0x71, name: "ADC", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0x29, name: "AND", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x25, name: "AND", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x35, name: "AND", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x2D, name: "AND", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x3D, name: "AND", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x39, name: "AND", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0x21, name: "AND", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0x31, name: "AND", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0x0A, name: "ASL", bytes: 1, addressing_mode: AddressingModes::Accumulator, cycles: 2},
        Instruction {opcode: 0x06, name: "ASL", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x16, name: "ASL", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x0E, name: "ASL", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x1E, name: "ASL", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x90, name: "BCC", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0xB0, name: "BCS", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0xF0, name: "BEQ", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0x24, name: "BIT", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x2C, name: "BIT", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x30, name: "BMI", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0xD0, name: "BNE", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0x10, name: "BPL", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0x00, name: "BRK", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 7},
        Instruction {opcode: 0x50, name: "BVC", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0x70, name: "BVS", bytes: 2, addressing_mode: AddressingModes::Relative, cycles: 2},
        Instruction {opcode: 0x18, name: "CLC", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xD8, name: "CLD", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x58, name: "CLI", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xB8, name: "CLV", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xC9, name: "CMP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xC5, name: "CMP", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xD5, name: "CMP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0xCD, name: "CMP", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xDD, name: "CMP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0xD9, name: "CMP", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0xC1, name: "CMP", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0xD1, name: "CMP", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0xE0, name: "CPX", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xE4, name: "CPX", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xEC, name: "CPX", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xC0, name: "CPY", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xC4, name: "CPY", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xCC, name: "CPY", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xC6, name: "DEC", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0xD6, name: "DEC", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0xCE, name: "DEC", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0xDE, name: "DEC", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0xCA, name: "DEX", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x88, name: "DEY", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x49, name: "EOR", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x45, name: "EOR", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x55, name: "EOR", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x4D, name: "EOR", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x5D, name: "EOR", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x59, name: "EOR", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0x41, name: "EOR", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0x51, name: "EOR", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0xE6, name: "INC", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0xF6, name: "INC", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0xEE, name: "INC", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0xFE, name: "INC", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0xE8, name: "INX", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xC8, name: "INY", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x4C, name: "JMP", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 3},
        Instruction {opcode: 0x6C, name: "JMP", bytes: 5, addressing_mode: AddressingModes::Indirect, cycles: 5},
        Instruction {opcode: 0x20, name: "JSR", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0xA9, name: "LDA", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xA5, name: "LDA", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xB5, name: "LDA", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0xAD, name: "LDA", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xBD, name: "LDA", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0xB9, name: "LDA", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0xA1, name: "LDA", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0xB1, name: "LDA", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0xA2, name: "LDX", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xA6, name: "LDX", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xB6, name: "LDX", bytes: 2, addressing_mode: AddressingModes::ZeroPageY, cycles: 4},
        Instruction {opcode: 0xAE, name: "LDX", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xBE, name: "LDX", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0xA0, name: "LDY", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xA4, name: "LDY", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xB4, name: "LDY", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0xAC, name: "LDY", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xBC, name: "LDY", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x4A, name: "LSR", bytes: 1, addressing_mode: AddressingModes::Accumulator, cycles: 2},
        Instruction {opcode: 0x46, name: "LSR", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x56, name: "LSR", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x4E, name: "LSR", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x5E, name: "LSR", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x1A, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x3A, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x5A, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x7A, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xDA, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xEA, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xFA, name: "NOP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x09, name: "ORA", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x05, name: "ORA", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x15, name: "ORA", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x0D, name: "ORA", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x1D, name: "ORA", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x19, name: "ORA", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0x01, name: "ORA", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0x11, name: "ORA", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0x48, name: "PHA", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 3},
        Instruction {opcode: 0x08, name: "PHP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 3},
        Instruction {opcode: 0x68, name: "PLA", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 4},
        Instruction {opcode: 0x28, name: "PLP", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 4},
        Instruction {opcode: 0x2A, name: "ROL", bytes: 1, addressing_mode: AddressingModes::Accumulator, cycles: 2},
        Instruction {opcode: 0x26, name: "ROL", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x36, name: "ROL", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x2E, name: "ROL", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x3E, name: "ROL", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x6A, name: "ROR", bytes: 1, addressing_mode: AddressingModes::Accumulator, cycles: 2},
        Instruction {opcode: 0x66, name: "ROR", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x76, name: "ROR", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x6E, name: "ROR", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x7E, name: "ROR", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x40, name: "RTI", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 6},
        Instruction {opcode: 0x60, name: "RTS", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 6},
        Instruction {opcode: 0xE9, name: "SBC", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xE5, name: "SBC", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0xF5, name: "SBC", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0xED, name: "SBC", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xFD, name: "SBC", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0xF9, name: "SBC", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 4},
        Instruction {opcode: 0xE1, name: "SBC", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0xF1, name: "SBC", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 5},
        Instruction {opcode: 0x38, name: "SEC", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xF8, name: "SED", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x78, name: "SEI", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x85, name: "STA", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x95, name: "STA", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x8D, name: "STA", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x9D, name: "STA", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 5},
        Instruction {opcode: 0x99, name: "STA", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 5},
        Instruction {opcode: 0x81, name: "STA", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 6},
        Instruction {opcode: 0x91, name: "STA", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 6},
        Instruction {opcode: 0x86, name: "STX", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x96, name: "STX", bytes: 2, addressing_mode: AddressingModes::ZeroPageY, cycles: 4},
        Instruction {opcode: 0x8E, name: "STX", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x84, name: "STY", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x94, name: "STY", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x8C, name: "STY", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0xAA, name: "TAX", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xA8, name: "TAY", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0xBA, name: "TSX", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x8A, name: "TXA", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x9A, name: "TXS", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},
        Instruction {opcode: 0x98, name: "TYA", bytes: 1, addressing_mode: AddressingModes::Implicit, cycles: 2},

        // Illegal Opcodes
        Instruction {opcode: 0x0b, name: "AAC", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x2b, name: "AAC", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x04, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x14, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x34, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x44, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x54, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x64, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 3},
        Instruction {opcode: 0x74, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x80, name: "DOP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x82, name: "DOP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0x89, name: "DOP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xC2, name: "DOP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xD4, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0xE2, name: "DOP", bytes: 2, addressing_mode: AddressingModes::Immediate, cycles: 2},
        Instruction {opcode: 0xF4, name: "DOP", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 4},
        Instruction {opcode: 0x27, name: "RLA", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x37, name: "RLA", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x2F, name: "RLA", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x3F, name: "RLA", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x3B, name: "RLA", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 7},
        Instruction {opcode: 0x23, name: "RLA", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 8},
        Instruction {opcode: 0x33, name: "RLA", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 8},
        Instruction {opcode: 0x07, name: "SLO", bytes: 2, addressing_mode: AddressingModes::ZeroPage, cycles: 5},
        Instruction {opcode: 0x17, name: "SLO", bytes: 2, addressing_mode: AddressingModes::ZeroPageX, cycles: 6},
        Instruction {opcode: 0x0F, name: "SLO", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 6},
        Instruction {opcode: 0x1F, name: "SLO", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 7},
        Instruction {opcode: 0x1B, name: "SLO", bytes: 3, addressing_mode: AddressingModes::AbsoluteY, cycles: 7},
        Instruction {opcode: 0x03, name: "SLO", bytes: 2, addressing_mode: AddressingModes::IndexedIndirectX, cycles: 8},
        Instruction {opcode: 0x13, name: "SLO", bytes: 2, addressing_mode: AddressingModes::IndirectIndexedY, cycles: 8},
        Instruction {opcode: 0x0C, name: "TOP", bytes: 3, addressing_mode: AddressingModes::Absolute, cycles: 4},
        Instruction {opcode: 0x1C, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x3C, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x5C, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0x7C, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0xDC, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        Instruction {opcode: 0xFC, name: "TOP", bytes: 3, addressing_mode: AddressingModes::AbsoluteX, cycles: 4},
        ]
    .into_iter()
    .map(|instruction| (instruction.opcode, instruction))
    .collect()
});

fn aac(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_a = cpu.register_a & operand;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    match cpu.get_flag_state(STATUS_FLAG_NEGATIVE) {
        FlagStates::SET => {
            cpu.set_flag(STATUS_FLAG_CARRY);
        }
        FlagStates::CLEAR => {
            cpu.clear_flag(STATUS_FLAG_CARRY);
        }
    }
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn adc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    let register_a_sign = cpu.register_a & 0b1000_0000;
    let operand_sign = operand & 0b1000_0000;
    let carry = cpu.get_flag_state(STATUS_FLAG_CARRY);
    let (temp_sum, overflow_occured_on_first_addition) = cpu.register_a.overflowing_add(operand);
    let (final_sum, overflow_occured_on_second_addition) = temp_sum.overflowing_add(carry as u8);
    cpu.register_a = final_sum;
    if overflow_occured_on_first_addition || overflow_occured_on_second_addition {
        cpu.set_flag(STATUS_FLAG_CARRY);
        cpu.set_flag(STATUS_FLAG_OVERFLOW);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY)
    };

    let result_sign = cpu.register_a & 0b1000_0000;
    if register_a_sign == operand_sign && result_sign != register_a_sign {
        cpu.set_flag(STATUS_FLAG_OVERFLOW);
    } else {
        cpu.clear_flag(STATUS_FLAG_OVERFLOW);
    }

    cpu.update_negative_flag(cpu.register_a);
    cpu.update_zero_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn and(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_a = cpu.register_a & operand;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn asl(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // code duplication, almost identical to lsr
    let instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_most_significant_bit = (operand & 0b1000_0000) >> 7;
    let result = operand << 1;

    match instruction.addressing_mode {
        AddressingModes::Accumulator => {
            cpu.register_a = result;
        }
        _ => {
            let operand_address = instruction.addressing_mode.get_operand_address(&cpu);
            cpu.mapper.borrow_mut().write_u8(operand_address, result);
        }
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);

    if operand_most_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }
    return instruction_result;
}

fn bcc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_CARRY) {
        FlagStates::CLEAR => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::SET => (),
    }
    return instruction_result;
}

fn bcs(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_CARRY) {
        FlagStates::SET => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::CLEAR => (),
    }
    return instruction_result;
}

fn beq(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_ZERO) {
        FlagStates::SET => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::CLEAR => (),
    }
    return instruction_result;
}

fn bit(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    let result = cpu.register_a & operand;

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(operand);

    if operand & 0b0100_0000 == 0b0100_0000 {
        cpu.set_flag(STATUS_FLAG_OVERFLOW);
    } else {
        cpu.clear_flag(STATUS_FLAG_OVERFLOW);
    }
    return instruction_result;
}

fn bmi(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_NEGATIVE) {
        FlagStates::SET => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::CLEAR => (),
    }
    return instruction_result;
}

fn bne(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_ZERO) {
        FlagStates::CLEAR => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::SET => (),
    }
    return instruction_result;
}

fn bpl(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_NEGATIVE) {
        FlagStates::CLEAR => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::SET => (),
    }
    return instruction_result;
}

fn bvc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_OVERFLOW) {
        FlagStates::CLEAR => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::SET => (),
    }
    return instruction_result;
}

fn bvs(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    match cpu.get_flag_state(STATUS_FLAG_OVERFLOW) {
        FlagStates::SET => {
            let distance = cpu.mapper.borrow().read_u8(cpu.program_counter);
            let page_crossed = cpu.branch_off_program_counter(distance);
            instruction_result.executed_cycles += 1;
            instruction_result.executed_cycles += page_crossed as u8;
        }
        FlagStates::CLEAR => (),
    }
    return instruction_result;
}

fn brk(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let interrupt_vector = cpu.mapper.borrow().read_u16(0xFFFE);
    cpu.stack_push_u16(cpu.program_counter.wrapping_add(1));
    cpu.stack_push(cpu.status | 0b0001_0000);
    cpu.set_flag(STATUS_FLAG_INTERRUPT_DISABLE);
    cpu.program_counter = interrupt_vector;
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn clc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.clear_flag(STATUS_FLAG_CARRY);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn cld(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.clear_flag(STATUS_FLAG_DECIMAL);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn cli(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.clear_flag(STATUS_FLAG_INTERRUPT_DISABLE);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn clv(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.clear_flag(STATUS_FLAG_OVERFLOW);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn cmp(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let (result, overflow_occured) = cpu
        .register_a
        .overflowing_sub(instruction.addressing_mode.get_operand(&cpu));

    if overflow_occured {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.set_flag(STATUS_FLAG_CARRY);
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn cpx(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // todo: remove code duplication, almost similar to cmp, cpy
    let (result, overflow_occured) = cpu
        .register_x
        .overflowing_sub(instruction.addressing_mode.get_operand(&cpu));

    if overflow_occured {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.set_flag(STATUS_FLAG_CARRY);
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn cpy(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // todo: remove code duplication, almost similar to cmp, cpx
    let (result, overflow_occured) = cpu
        .register_y
        .overflowing_sub(instruction.addressing_mode.get_operand(&cpu));

    if overflow_occured {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.set_flag(STATUS_FLAG_CARRY);
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn dec(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let address = instruction.addressing_mode.get_operand_address(&cpu);
    let result = cpu.mapper.borrow().read_u8(address).wrapping_sub(1);
    cpu.mapper.borrow_mut().write_u8(address, result);
    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn dex(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_x = cpu.register_x.wrapping_sub(1);
    cpu.update_zero_flag(cpu.register_x);
    cpu.update_negative_flag(cpu.register_x);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn dey(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_y = cpu.register_y.wrapping_sub(1);
    cpu.update_zero_flag(cpu.register_y);
    cpu.update_negative_flag(cpu.register_y);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn dop(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    nop(instruction, cpu);
    nop(instruction, cpu)
}

fn eor(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_a = cpu.register_a ^ operand;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn lda(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    // todo: remove duplicate code, same as ldx() and ldy()
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_a = operand;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn ldx(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_x = operand;
    cpu.update_zero_flag(cpu.register_x);
    cpu.update_negative_flag(cpu.register_x);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn ldy(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_y = operand;
    cpu.update_zero_flag(cpu.register_y);
    cpu.update_negative_flag(cpu.register_y);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn lsr(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_least_significant_bit = operand & 0b0000_0001;
    let result = operand >> 1;

    match instruction.addressing_mode {
        AddressingModes::Accumulator => {
            cpu.register_a = result;
        }
        _ => {
            let address = instruction.addressing_mode.get_operand_address(&cpu);
            cpu.mapper.borrow_mut().write_u8(address, result);
        }
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);

    if operand_least_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn ora(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = instruction.addressing_mode.get_operand(&cpu);
    cpu.register_a = cpu.register_a | operand;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn pha(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.stack_push(cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn php(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.stack_push(cpu.status | 0b0001_0000);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn plp(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.status = cpu.stack_pop() | 0b0010_0000;
    // NesDev reference says that this flag should be set from stack,
    // but the test suite only passes if I clear it here.
    cpu.clear_flag(STATUS_FLAG_BREAK_COMMAND);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn pla(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_a = cpu.stack_pop();

    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn inc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let address = instruction.addressing_mode.get_operand_address(&cpu);
    let result = cpu.mapper.borrow().read_u8(address).wrapping_add(1);
    cpu.mapper.borrow_mut().write_u8(address, result);
    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn inx(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    cpu.update_zero_flag(cpu.register_x);
    cpu.update_negative_flag(cpu.register_x);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn iny(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_y = cpu.register_y.wrapping_add(1);
    cpu.update_zero_flag(cpu.register_y);
    cpu.update_negative_flag(cpu.register_y);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn rla(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // Executes a rol followed by and
    let carry = cpu.get_flag_state(STATUS_FLAG_CARRY);

    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_most_significant_bit = (operand & 0b1000_0000) >> 7;
    let mut result = operand << 1;

    match carry {
        FlagStates::SET => {
            result = result | 0b0000_0001;
        }
        FlagStates::CLEAR => {
            result = result & 0b1111_1110;
        }
    }

    let address = instruction.addressing_mode.get_operand_address(&cpu);
    cpu.mapper.borrow_mut().write_u8(address, result);

    cpu.register_a = cpu.register_a & result;

    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);

    if operand_most_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }

    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn rol(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let carry = cpu.get_flag_state(STATUS_FLAG_CARRY);

    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_most_significant_bit = (operand & 0b1000_0000) >> 7;
    let mut result = operand << 1;

    match carry {
        FlagStates::SET => {
            result = result | 0b0000_0001;
        }
        FlagStates::CLEAR => {
            result = result & 0b1111_1110;
        }
    }

    match instruction.addressing_mode {
        AddressingModes::Accumulator => {
            cpu.register_a = result;
        }
        _ => {
            let address = instruction.addressing_mode.get_operand_address(&cpu);
            cpu.mapper.borrow_mut().write_u8(address, result);
        }
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);

    if operand_most_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn ror(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let carry = cpu.get_flag_state(STATUS_FLAG_CARRY);

    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_least_significant_bit = operand & 0b0000_0001;
    let mut result = operand >> 1;

    match carry {
        FlagStates::SET => {
            result = result | 0b1000_0000;
        }
        FlagStates::CLEAR => {
            result = result & 0b0111_1111;
        }
    }

    match instruction.addressing_mode {
        AddressingModes::Accumulator => {
            cpu.register_a = result;
        }
        _ => {
            let address = instruction.addressing_mode.get_operand_address(&cpu);
            cpu.mapper.borrow_mut().write_u8(address, result);
        }
    }

    cpu.update_zero_flag(result);
    cpu.update_negative_flag(result);

    if operand_least_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn rti(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.status = cpu.stack_pop() | 0b0010_0000 & 0b1110_1111;
    cpu.program_counter = cpu.stack_pop_u16().wrapping_sub(1);
    cpu.clear_flag(STATUS_FLAG_BREAK_COMMAND);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn rts(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.program_counter = cpu.stack_pop_u16();
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn nop(instruction: &Instruction, _cpu: &mut CPU) -> InstructionResult {
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn jmp(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.program_counter = instruction.addressing_mode.get_operand_address(&cpu);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn jsr(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // Program counter is incremented instead of decremented as requested in the nesdev reference
    cpu.stack_push_u16(cpu.program_counter.wrapping_add(1));
    cpu.program_counter = instruction.addressing_mode.get_operand_address(&cpu);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn sbc(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };
    let operand = !instruction.addressing_mode.get_operand(&cpu);
    let register_a_sign = cpu.register_a & 0b1000_0000;
    let operand_sign = operand & 0b1000_0000;
    let carry = cpu.get_flag_state(STATUS_FLAG_CARRY) as u8;
    let (temp_sum, overflow_occured_on_first_addition) = cpu.register_a.overflowing_add(operand);
    let (final_sum, overflow_occured_on_second_addition) = temp_sum.overflowing_add(carry);
    cpu.register_a = final_sum;
    if overflow_occured_on_first_addition || overflow_occured_on_second_addition {
        cpu.set_flag(STATUS_FLAG_CARRY);
        cpu.set_flag(STATUS_FLAG_OVERFLOW);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY)
    };

    let result_sign = cpu.register_a & 0b1000_0000;
    if register_a_sign == operand_sign && result_sign != register_a_sign {
        cpu.set_flag(STATUS_FLAG_OVERFLOW);
    } else {
        cpu.clear_flag(STATUS_FLAG_OVERFLOW);
    }

    cpu.update_negative_flag(cpu.register_a);
    cpu.update_zero_flag(cpu.register_a);
    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    return instruction_result;
}

fn sec(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.set_flag(STATUS_FLAG_CARRY);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn sed(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.set_flag(STATUS_FLAG_DECIMAL);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn slo(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    // executes an asl followed by ora
    let operand = instruction.addressing_mode.get_operand(&cpu);
    let operand_most_significant_bit = (operand & 0b1000_0000) >> 7;
    let result = operand << 1;

    match instruction.addressing_mode {
        AddressingModes::Accumulator => {
            cpu.register_a = result;
        }
        _ => {
            let operand_address = instruction.addressing_mode.get_operand_address(&cpu);
            cpu.mapper.borrow_mut().write_u8(operand_address, result);
        }
    }

    if operand_most_significant_bit == 1 {
        cpu.set_flag(STATUS_FLAG_CARRY);
    } else {
        cpu.clear_flag(STATUS_FLAG_CARRY);
    }

    cpu.register_a = cpu.register_a | result;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn sei(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.set_flag(STATUS_FLAG_INTERRUPT_DISABLE);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn sta(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let address = instruction.addressing_mode.get_operand_address(&cpu);
    cpu.mapper.borrow_mut().write_u8(address, cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn stx(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let address = instruction.addressing_mode.get_operand_address(&cpu);
    cpu.mapper.borrow_mut().write_u8(address, cpu.register_x);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
fn sty(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let address = instruction.addressing_mode.get_operand_address(&cpu);
    cpu.mapper.borrow_mut().write_u8(address, cpu.register_y);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn tax(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_x = cpu.register_a;
    cpu.update_zero_flag(cpu.register_x);
    cpu.update_negative_flag(cpu.register_x);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn tay(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_y = cpu.register_a;

    cpu.update_zero_flag(cpu.register_y);
    cpu.update_negative_flag(cpu.register_y);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn top(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    let mut instruction_result = InstructionResult {
        executed_cycles: instruction.cycles,
    };

    instruction_result.executed_cycles += instruction.addressing_mode.is_page_crossed(cpu) as u8;
    nop(instruction, cpu);
    nop(instruction, cpu);
    nop(instruction, cpu);
    return instruction_result;
}

fn tsx(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_x = cpu.stack_pointer;
    cpu.update_zero_flag(cpu.register_x);
    cpu.update_negative_flag(cpu.register_x);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn txa(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_a = cpu.register_x;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn txs(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.stack_pointer = cpu.register_x;
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}

fn tya(instruction: &Instruction, cpu: &mut CPU) -> InstructionResult {
    cpu.register_a = cpu.register_y;
    cpu.update_zero_flag(cpu.register_a);
    cpu.update_negative_flag(cpu.register_a);
    return InstructionResult {
        executed_cycles: instruction.cycles,
    };
}
