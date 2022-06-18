use crate::config;
use crate::cpu;
use crate::cpu::register;
use crate::ram;

/* instructions handled by the cpu, structured as a tuple in the format
(name, opcode, number of bytes, execution function that returns the number of cycles it took to complete)
using this document for functionality reference https://www.nesdev.org/obelisk-6502-guide/reference.html

the name field will indicate the addressing mode with the following codes, if not specified its implicit/accumulator/etc
I - indirect
Z - zero page
ZX - zero page X
A - absolute
AX - absolute X
AY - absolute Y
IX - indirect X
IY - indirect Y
*/
pub static OPCODES: [(&str, u8, i32, fn(&mut config::Emulator) -> u32); 8] = [
    ("ADC - I",  0x69,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        adc(emulator, value);
        return 2;
    }),
    ("ADC - Z",  0x65,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::ZeroPage { address });
        adc(emulator, value);
        return 3;
    }),
    ("ADC - ZX",  0x75,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - A",  0x6D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::Absolute { address });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - AX",  0x7D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - AY",  0x79,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - IX",  0x61,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 6;
    }),
    ("ADC - IY",  0x71,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = emulator.ram.read_with_addressing_mode(ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        adc(emulator, value);
        return 5 + add_cycle as u32;
    }),
];

fn adc(emulator: &mut config::Emulator, value: u8) {
    let total: u16 = emulator.cpu.registers.a as u16 
        + value as u16
        + emulator.cpu.registers.status.contains(register::Status::C) as u16;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, total > 0xFF);
    emulator.cpu.registers.status.set(register::Status::Z, total & 0xFF == 0);
    emulator.cpu.registers.status.set(register::Status::V, (emulator.cpu.registers.a as u16 ^ total) & (value as u16 ^ total) & 0x80 == 0x80); // if pos + pos = neg or neg + neg = pos, explanations here https://forums.nesdev.org/viewtopic.php?t=6331
    emulator.cpu.registers.status.set(register::Status::N, total & 0x80 == 0x80);
    
    // registers
    emulator.cpu.registers.a = (total & 0xFF) as u8;
}