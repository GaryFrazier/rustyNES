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

    // CMP - Compare
    ("CMP - I",  0xC9,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        cmp(emulator, value);
        return 2;
    }),
    ("CMP - Z",  0xC5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        cmp(emulator, value);
        return 3;
    }),
    ("CMP - ZX",  0xD5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 4;
    }),
    ("CMP - A",  0xCD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        cmp(emulator, value);
        return 4;
    }),
    ("CMP - AX",  0xDD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("CMP - AY",  0xD9,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        cmp(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("CMP - IX",  0xC1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 6;
    }),
    ("CMP - IY",  0xD1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        cmp(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // CPX - Compare X Register
    ("CPX - I",  0xE0,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        cpx(emulator, value);
        return 2;
    }),
    ("CPX - Z",  0xE4,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        cpx(emulator, value);
        return 3;
    }),
    ("CPX - A",  0xEC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        cpx(emulator, value);
        return 4;
    }),

    // CPY - Compare Y Register
    ("CPY - I",  0xC0,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        cpx(emulator, value);
        return 2;
    }),
    ("CPY - Z",  0xC4,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        cpx(emulator, value);
        return 3;
    }),
    ("CPY - A",  0xCC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        cpx(emulator, value);
        return 4;
    }),

    // DEC - Decrement Memory
    ("DEC - Z",  0xC6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        let result = dec(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("DEC - ZX",  0xD6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = dec(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("DEC - A",  0xCE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        let result = dec(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("DEC - AX",  0xDE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = dec(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    //DEX - Decrement X Register
    ("DEX",  0xCA,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.x = dec(emulator, emulator.cpu.registers.x);
        return 2;
    }),

    //DEY - Decrement Y Register
    ("DEY",  0x88,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.y = dec(emulator, emulator.cpu.registers.y);
        return 2;
    }),

    // EOR - Exclusive OR
    ("EOR - I",  0x49,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        eor(emulator, value);
        return 2;
    }),
    ("EOR - Z",  0x45,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        eor(emulator, value);
        return 3;
    }),
    ("EOR - ZX",  0x55,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 4;
    }),
    ("EOR - A",  0x4D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        eor(emulator, value);
        return 4;
    }),
    ("EOR - AX",  0x5D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("EOR - AY",  0x59,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        eor(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("EOR - IX",  0x41,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 6;
    }),
    ("EOR - IY",  0x51,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        eor(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // INC - Increment Memory
    ("INC - Z",  0xE6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPage { address });
        let result = inc(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("INC - ZX",  0xF6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = inc(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("INC - A",  0xEE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::Absolute { address });
        let result = inc(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("INC - AX",  0xFE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = ram::read_with_addressing_mode(&mut emulator.cpu.memory, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = inc(emulator, value);
        ram::write_with_addressing_mode(&mut emulator.cpu.memory, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    //INC - Decrement X Register
    ("INC",  0xE8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.x = inc(emulator, emulator.cpu.registers.x);
        return 2;
    }),

    //INC - Decrement Y Register
    ("INC",  0xC8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.y = inc(emulator, emulator.cpu.registers.y);
        return 2;
    }),

    // JMP - Jump
    ("JMP - A",  0x4C,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        jmp(emulator, address);
        return 3;
    }),
    ("JMP - Indirect",  0x6C,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let value = ram::read_u16(&mut emulator.cpu.memory, address.into());
        jmp(emulator, value);
        return 5;
    }),

    // JSR - Jump to Subroutine
    ("JSR - A",  0x20,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        jsr(emulator, address);
        return 6;
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

fn eor(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a ^ value;

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

fn cmp(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a as i16 - value as i16;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, result >= 0);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result < 0);
}

fn cpx(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.x as i16 - value as i16;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, result >= 0);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result < 0);
}

fn cpy(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.y as i16 - value as i16;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, result >= 0);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result < 0);
}

fn dec(emulator: &mut config::Emulator, value: u8) -> u8 {
    let signedValue = value as i8;
    let result: i8 = signedValue - 1;

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result < 0);
    
    // result
    return result as u8;
}

fn inc(emulator: &mut config::Emulator, value: u8) -> u8 {
    let result: u8 = value + 1;

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
    
    // result
    return result;
}

fn jmp(emulator: &mut config::Emulator, address: u16) {
    emulator.cpu.registers.pc = address;
}

fn jsr(emulator: &mut config::Emulator, address: u16) {
    emulator.cpu.registers.pc -= 1;
    cpu::write_stack_u16(emulator, emulator.cpu.registers.pc);
    jmp(emulator, address);
}