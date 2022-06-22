use crate::config;
use crate::cpu;
use crate::cpu::register;
use crate::ram;

/* instructions handled by the cpu, structured as a tuple in the format
(name, opcode, number of bytes, execution function that returns the number of cycles it took to complete)
using this document for functionality reference https://www.nesdev.org/obelisk-6502-guide/reference.html

the name field will indicate the addressing mode with the following codes, if not specified its implicit/accumulator/etc
A - accumulator
I - indirect
Z - zero page
ZX - zero page X
A - absolute
AX - absolute X
AY - absolute Y
IX - indirect X
IY - indirect Y
R - relative
*/
pub static OPCODES: [(&str, u8, i32, fn(&mut config::Emulator) -> u32); 33] = [
    // ADC - Add with Carry
    ("ADC - I",  0x69,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        adc(emulator, value);
        return 2;
    }),
    ("ADC - Z",  0x65,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        adc(emulator, value);
        return 3;
    }),
    ("ADC - ZX",  0x75,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - A",  0x6D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - AX",  0x7D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - AY",  0x79,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - IX",  0x61,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 6;
    }),
    ("ADC - IY",  0x71,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        adc(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // AND - Logical AND
    ("AND - I",  0x29,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        and(emulator, value);
        return 2;
    }),
    ("AND - Z",  0x25,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        and(emulator, value);
        return 3;
    }),
    ("AND - ZX",  0x35,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 4;
    }),
    ("AND - A",  0x2D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        and(emulator, value);
        return 4;
    }),
    ("AND - AX",  0x3D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("AND - AY",  0x39,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        and(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("AND - IX",  0x21,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 6;
    }),
    ("AND - IY",  0x31,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        and(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // ASL - Arithmetic Shift Left
    ("ASL - A",  0x0A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = asl(emulator, emulator.cpu.registers.a);
        return 2;
    }),
    ("ASL - Z",  0x06,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        let result = asl(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("ASL - ZX",  0x16,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = asl(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("ASL - A",  0x0E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        let result = asl(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("ASL - AX",  0x1E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = asl(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    // BCC - Branch if Carry Clear
    ("BCC - R",  0x90,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, !emulator.cpu.registers.status.contains(register::Status::C));
    }),
    
    // BCC - Branch if Carry Set
    ("BCS - R",  0xB0,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, emulator.cpu.registers.status.contains(register::Status::C));
    }),

     // BEQ - Branch if Equal
    ("BEQ - R",  0xF0,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, emulator.cpu.registers.status.contains(register::Status::Z));
    }),

     // BMI - Branch if Minus
    ("BMI - R",  0x30,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, emulator.cpu.registers.status.contains(register::Status::N));
    }),

    // BNE - Branch if Not Equal
    ("BNE - R",  0xD0,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, !emulator.cpu.registers.status.contains(register::Status::Z));
    }),

    // BPL - Branch if Positive
    ("BPL - R",  0x10,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, !emulator.cpu.registers.status.contains(register::Status::N));
    }),

    // BVC - Branch if Overflow Clear
    ("BVC - R",  0x50,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, !emulator.cpu.registers.status.contains(register::Status::V));
    }),

    // BVS - Branch if Overflow Set
    ("BVC - R",  0x70,  2, |emulator: &mut config::Emulator| -> u32 {
        return relative_branch(emulator, emulator.cpu.registers.status.contains(register::Status::V));
    }),

    // BIT - Bit Test
    ("BIT - Z",  0x24,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        bit_test(emulator, value);
        return 3;
    }),
    ("BIT - A",  0x2C,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        bit_test(emulator, value);
        return 4;
    }),

    // BRK - Force Interrupt
    ("BRK",  0x00,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::B, true);
        return 7;
    }),
    
    // CLC - Clear Carry Flag
    ("CLC",  0x18,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::C, false);
        return 2;
    }),

    // CLD - Clear Decimal Mode
    ("CLD",  0xD8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::D, false);
        return 2;
    }),

    // CLI - Clear Interrupt Disable
    ("CLI",  0x58,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::I, false);
        return 2;
    }),

    // CLV - Clear Overflow Flag
    ("CLV",  0xB8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::V, false);
        return 2;
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

fn and(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a & value;

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
    
    // registers
    emulator.cpu.registers.a = result;
}

fn asl(emulator: &mut config::Emulator, value: u8) -> u8 {
    let result: u8 = value << 1;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, value & 0x80 == 0x80);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
    
    // result
    return result;
}

fn bit_test(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a & value; // result not kept

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::V, result & 0x40 == 0x40);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
}

// returns cycles
fn relative_branch(emulator: &mut config::Emulator, should_branch: bool) -> u32 {
    if should_branch {
        let offset = cpu::read_program_byte(emulator) as i8; // signed
        let (address, page_change) = ram::relative_offset_page_change(emulator.cpu.registers.pc, offset);
        emulator.cpu.registers.pc = address;
        return 3 + page_change as u32;
    }
    
    return 2;
}