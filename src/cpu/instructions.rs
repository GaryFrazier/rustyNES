use crate::config;
use crate::cpu;
use crate::cpu::register;
use crate::ram;

/* instructions handled by the cpu, structured as a tuple in the format
(name, opcode, number of bytes, execution function that returns the number of cycles it took to complete)
using this document for functionality reference https://www.nesdev.org/obelisk-6502-guide/reference.html

the name field will indicate the addressing mode with the following codes, if not specified its implicit/accumulator/etc
A - accumulator
I - Immediate
Z - zero page
ZX - zero page X
ZY - zero page Y
A - absolute
AX - absolute X
AY - absolute Y
IX - indirect X
IY - indirect Y
R - relative
*/
pub static OPCODES: [(&str, u8, i32, fn(&mut config::Emulator) -> u32); 158] = [
    // ADC - Add with Carry
    ("ADC - I",  0x69,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        adc(emulator, value);
        return 2;
    }),
    ("ADC - Z",  0x65,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        adc(emulator, value);
        return 3;
    }),
    ("ADC - ZX",  0x75,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - A",  0x6D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        adc(emulator, value);
        return 4;
    }),
    ("ADC - AX",  0x7D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - AY",  0x79,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        adc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ADC - IX",  0x61,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        adc(emulator, value);
        return 6;
    }),
    ("ADC - IY",  0x71,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        and(emulator, value);
        return 3;
    }),
    ("AND - ZX",  0x35,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 4;
    }),
    ("AND - A",  0x2D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        and(emulator, value);
        return 4;
    }),
    ("AND - AX",  0x3D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("AND - AY",  0x39,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        and(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("AND - IX",  0x21,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        and(emulator, value);
        return 6;
    }),
    ("AND - IY",  0x31,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = asl(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("ASL - ZX",  0x16,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = asl(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("ASL - A",  0x0E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = asl(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("ASL - AX",  0x1E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = asl(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        bit_test(emulator, value);
        return 3;
    }),
    ("BIT - A",  0x2C,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        bit_test(emulator, value);
        return 4;
    }),

    // BRK - Force Interrupt
    ("BRK",  0x00,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::B, true);
        cpu::irq(emulator);
        return 0; // cycles handled by irq
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        cmp(emulator, value);
        return 3;
    }),
    ("CMP - ZX",  0xD5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 4;
    }),
    ("CMP - A",  0xCD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        cmp(emulator, value);
        return 4;
    }),
    ("CMP - AX",  0xDD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("CMP - AY",  0xD9,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        cmp(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("CMP - IX",  0xC1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        cmp(emulator, value);
        return 6;
    }),
    ("CMP - IY",  0xD1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        cpx(emulator, value);
        return 3;
    }),
    ("CPX - A",  0xEC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        cpx(emulator, value);
        return 4;
    }),

    // CPY - Compare Y Register
    ("CPY - I",  0xC0,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        cpy(emulator, value);
        return 2;
    }),
    ("CPY - Z",  0xC4,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        cpy(emulator, value);
        return 3;
    }),
    ("CPY - A",  0xCC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        cpy(emulator, value);
        return 4;
    }),

    // DEC - Decrement Memory
    ("DEC - Z",  0xC6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = dec(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("DEC - ZX",  0xD6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = dec(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("DEC - A",  0xCE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = dec(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("DEC - AX",  0xDE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = dec(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
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
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        eor(emulator, value);
        return 3;
    }),
    ("EOR - ZX",  0x55,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 4;
    }),
    ("EOR - A",  0x4D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        eor(emulator, value);
        return 4;
    }),
    ("EOR - AX",  0x5D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("EOR - AY",  0x59,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        eor(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("EOR - IX",  0x41,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 6;
    }),
    ("EOR - IY",  0x51,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        eor(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // INC - Increment Memory
    ("INC - Z",  0xE6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = inc(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("INC - ZX",  0xF6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = inc(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("INC - A",  0xEE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = inc(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("INC - AX",  0xFE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = inc(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
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
        let mut address = cpu::read_program_word(emulator);

        if address & 0xFF == 0xFF {
            let new_low = ram::read_u8(cpu::mapped_address, &mut emulator.cpu.memory, address.into());
            let high = address >> 8;
            address += 1;
            address = address & 0xFF;
            address = address | ((high as u16) << 8);
            let new_high = ram::read_u8(cpu::mapped_address, &mut emulator.cpu.memory, address.into());
            println!("{}",address);
            let calced_value = ((new_high as u16) << 8) | new_low as u16;
            jmp(emulator, calced_value);
            return 5;
        }

        let value = ram::read_u16(cpu::mapped_address, &mut emulator.cpu.memory, address.into());
        jmp(emulator, value);
        return 5;
    }),

    // JSR - Jump to Subroutine
    ("JSR - A",  0x20,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        jsr(emulator, address);
        return 6;
    }),

    // LDA - Load Accumulator
    ("LDA - I",  0xA9,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        lda(emulator, value);
        return 2;
    }),
    ("LDA - Z",  0xA5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        lda(emulator, value);
        return 3;
    }),
    ("LDA - ZX",  0xB5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        lda(emulator, value);
        return 4;
    }),
    ("LDA - A",  0xAD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        lda(emulator, value);
        return 4;
    }),
    ("LDA - AX",  0xBD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        lda(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("LDA - AY",  0xB9,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        lda(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("LDA - IX",  0xA1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        lda(emulator, value);
        return 6;
    }),
    ("LDA - IY",  0xB1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        lda(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // LDX - Load X
    ("LDX - I",  0xA2,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        ldx(emulator, value);
        return 2;
    }),
    ("LDX - Z",  0xA6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        ldx(emulator, value);
        return 3;
    }),
    ("LDX - ZY",  0xB6,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageY { address, y: emulator.cpu.registers.y });
        ldx(emulator, value);
        return 4;
    }),
    ("LDX - A",  0xAE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        ldx(emulator, value);
        return 4;
    }),
    ("LDX - AY",  0xBE,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        ldx(emulator, value);
        return 4 + add_cycle as u32;
    }),

     // LDY - Load Y
    ("LDY - I",  0xA0,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        ldy(emulator, value);
        return 2;
    }),
    ("LDY - Z",  0xA4,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        ldy(emulator, value);
        return 3;
    }),
    ("LDY - ZX",  0xB4,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        ldy(emulator, value);
        return 4;
    }),
    ("LDY - A",  0xAC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        ldy(emulator, value);
        return 4;
    }),
    ("LDY - AX",  0xBC,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        ldy(emulator, value);
        return 4 + add_cycle as u32;
    }),

    // LSR - Logical Shift Right
    ("LSR - A",  0x4A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = lsr(emulator, emulator.cpu.registers.a);
        return 2;
    }),
    ("LSR - Z",  0x46,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = lsr(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("LSR - ZX",  0x56,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = lsr(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("LSR - A",  0x4E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = lsr(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("LSR - AX",  0x5E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = lsr(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    // NOP - No Operation
    ("NOP",  0xEA,  1, |_: &mut config::Emulator| -> u32 {
        return 2;
    }),

    // ORA - Inclusive OR
    ("ORA - I",  0x09,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        ora(emulator, value);
        return 2;
    }),
    ("ORA - Z",  0x05,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        ora(emulator, value);
        return 3;
    }),
    ("ORA - ZX",  0x15,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        ora(emulator, value);
        return 4;
    }),
    ("ORA - A",  0x0D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        ora(emulator, value);
        return 4;
    }),
    ("ORA - AX",  0x1D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        eor(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ORA - AY",  0x19,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        ora(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("ORA - IX",  0x01,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        ora(emulator, value);
        return 6;
    }),
    ("ORA - IY",  0x11,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        ora(emulator, value);
        return 5 + add_cycle as u32;
    }),

    // PHA - Push Accumulator
    ("PHA",  0x48,  1, |emulator: &mut config::Emulator| -> u32 {
        cpu::write_stack_u8(emulator, emulator.cpu.registers.a);
        return 3;
    }),

    // PHP - Push Processor Status
    ("PHP",  0x08,  1, |emulator: &mut config::Emulator| -> u32 {
        cpu::write_stack_u8(emulator, emulator.cpu.registers.status.bits());
        return 3;
    }),

    // PLA - Pull Accumulator
    ("PLA",  0x68,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = cpu::read_stack_u8(emulator);
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.a == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.a & 0x80 == 0x80);
        return 4;
    }),

    // PLP - Pull Processor Status
    ("PLP",  0x28,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status = register::Status::from_bits(cpu::read_stack_u8(emulator)).unwrap();
        return 4;
    }),

    // ROL - Rotate Left
    ("ROL - A",  0x2A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = rol(emulator, emulator.cpu.registers.a);
        return 2;
    }),
    ("ROL - Z",  0x26,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = rol(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("ROL - ZX",  0x36,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = rol(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("ROL - A",  0x2E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = rol(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("ROL - AX",  0x3E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = rol(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    // ROR - Rotate Right
    ("ROR - A",  0x6A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = ror(emulator, emulator.cpu.registers.a);
        return 2;
    }),
    ("ROR - Z",  0x66,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        let result = ror(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 5;
    }),
    ("ROR - ZX",  0x76,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        let result = ror(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 6;
    }),
    ("ROR - A",  0x6E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        let result = ror(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 6;
    }),
    ("ROR - AX",  0x7E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        let result = ror(emulator, value);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        return 7;
    }),

    // RTI - Return From Interrupt
    ("RTI",  0x40,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status = register::Status::from_bits(cpu::read_stack_u8(emulator)).unwrap();
        emulator.cpu.registers.pc = cpu::read_stack_u16(emulator);
        return 6;
    }),

    // RTS - Return From Subroutine
    ("RTS",  0x60,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.pc = cpu::read_stack_u16(emulator);
        emulator.cpu.registers.pc += 1;
        return 6;
    }),

    // SBC - Subtract with Carry
    ("SBC - I",  0xE9,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        sbc(emulator, value);
        return 2;
    }),
    ("SBC - Z",  0xE5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPage { address });
        sbc(emulator, value);
        return 3;
    }),
    ("SBC - ZX",  0xF5,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        sbc(emulator, value);
        return 4;
    }),
    ("SBC - A",  0xED,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::Absolute { address });
        sbc(emulator, value);
        return 4;
    }),
    ("SBC - AX",  0xFD,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x });
        sbc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("SBC - AY",  0xF9,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y });
        sbc(emulator, value);
        return 4 + add_cycle as u32;
    }),
    ("SBC - IX",  0xE1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, _) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x });
        sbc(emulator, value);
        return 6;
    }),
    ("SBC - IY",  0xF1,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let (value, add_cycle) = cpu::read_with_addressing_mode(emulator, cpu::mapped_address, ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.y });
        sbc(emulator, value);
        return 5 + add_cycle as u32;
    }),

     // SEC - Set Carry Flag
    ("SEC",  0x38,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::C, true);
        return 2;
    }),

    // SED - Set Decimal Flag
    ("SED",  0xF8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::D, true);
        return 2;
    }),

    // SEI - Set Interrupt Disable
    ("SEI",  0x78,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.status.set(register::Status::I, true);
        return 2;
    }),

    // STA - Store Accumulator
    ("STA - Z",  0x85,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::ZeroPage { address });
        return 3;
    }),
    ("STA - ZX",  0x95,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 4;
    }),
    ("STA - A",  0x8D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::Absolute { address });
        return 4;
    }),
    ("STA - AX",  0x9D,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::AbsoluteX { address, x: emulator.cpu.registers.x  });
        return 5;
    }),
    ("STA - AY",  0x99,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::AbsoluteY { address, y: emulator.cpu.registers.y  });
        return 5;
    }),
    ("STA - IX",  0x81,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x  });
        return 6;
    }),
    ("STA - IY",  0x91,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.a], ram::AddressingMode::IndirectY { address, y: emulator.cpu.registers.x  });
        return 6;
    }),

    // STX - Store X
    ("STX - Z",  0x86,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.x], ram::AddressingMode::ZeroPage { address });
        return 3;
    }),
    ("STX - ZY",  0x96,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.x], ram::AddressingMode::ZeroPageY { address, y: emulator.cpu.registers.y });
        return 4;
    }),
    ("STX - A",  0x8E,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.x], ram::AddressingMode::Absolute { address });
        return 4;
    }),

    // STY - Store Y
    ("STY - Z",  0x84,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.y], ram::AddressingMode::ZeroPage { address });
        return 3;
    }),
    ("STY - ZX",  0x94,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.y], ram::AddressingMode::ZeroPageX { address, x: emulator.cpu.registers.x });
        return 4;
    }),
    ("STY - A",  0x8C,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[emulator.cpu.registers.y], ram::AddressingMode::Absolute { address });
        return 4;
    }),

    // TAX - Transfer Accumulator to X
    ("TAX",  0xAA,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.x = emulator.cpu.registers.a;
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.x == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.x & 0x80 == 0x80);
        return 2;
    }),

    // TAY - Transfer Accumulator to Y
    ("TAY",  0xA8,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.y = emulator.cpu.registers.a;
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.y == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.y & 0x80 == 0x80);
        return 2;
    }),

    // TSX - Transfer Stack Pointer to X
    ("TSX",  0xBA,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.x = emulator.cpu.registers.sp;
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.x == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.x & 0x80 == 0x80);
        return 2;
    }),

    // TXA - Transfer X to Accumulator
    ("TXA",  0x8A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = emulator.cpu.registers.x;
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.a == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.a & 0x80 == 0x80);
        return 2;
    }),

    // TXS - Transfer X to Stack Pointer
    ("TSX",  0x9A,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.sp = emulator.cpu.registers.x;
        return 2;
    }),

    // TYA - Transfer Y to Accumulator
    ("TYA",  0x98,  1, |emulator: &mut config::Emulator| -> u32 {
        emulator.cpu.registers.a = emulator.cpu.registers.y;
        emulator.cpu.registers.status.set(register::Status::Z, emulator.cpu.registers.a == 0);
        emulator.cpu.registers.status.set(register::Status::N, emulator.cpu.registers.a & 0x80 == 0x80);
        return 2;
    }),

    ///////////////////// UNOFFICAL OPCODES

    // AAC
    ("AAC - I - 1",  0x0b,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        aac(emulator, value);
        return 2;
    }),
    ("AAC - I - 2",  0x2b,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        aac(emulator, value);
        return 2;
    }),

    // AAX
    ("AAX - Z",  0x87,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let result = aax(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPage { address });
        return 3;
    }),
    ("AAX - ZY",  0x97,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let result = aax(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::ZeroPageY { address, y: emulator.cpu.registers.y });
        return 4;
    }),
    ("AAX - A",  0x8F,  3, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_word(emulator);
        let result = aax(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::Absolute { address });
        return 4;
    }),
    ("AAX - IX",  0x83,  2, |emulator: &mut config::Emulator| -> u32 {
        let address = cpu::read_program_byte(emulator);
        let result = aax(emulator);
        cpu::write_with_addressing_mode(emulator, cpu::mapped_address, &[result], ram::AddressingMode::IndirectX { address, x: emulator.cpu.registers.x  });
        return 6;
    }),

    // ARR
    ("ARR - I",  0x6b,  2, |emulator: &mut config::Emulator| -> u32 {
        let value = cpu::read_program_byte(emulator);
        arr(emulator, value);
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

fn sbc(emulator: &mut config::Emulator, value: u8) {
    let total: u16 = emulator.cpu.registers.a as u16 
        + (value as u16 ^ 0x00FF) // convert to negative to make this addition easier
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

fn ora(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a | value;

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

fn lsr(emulator: &mut config::Emulator, value: u8) -> u8 {
    let result: u8 = value >> 1;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, value & 0x1 == 0x1);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
    
    // result
    return result;
}

fn rol(emulator: &mut config::Emulator, value: u8) -> u8 {
    let carry: bool = value & 0x80 == 0x80;
    let result: u8 = (value << 1) | (emulator.cpu.registers.status.contains(register::Status::C) as u8 & 0x1);

    // flags
    emulator.cpu.registers.status.set(register::Status::C, carry);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
    
    // result
    return result;
}

fn ror(emulator: &mut config::Emulator, value: u8) -> u8 {
    let carry: bool = value & 0x1 == 0x1;
    let result: u8 = (value >> 1) | ((emulator.cpu.registers.status.contains(register::Status::C) as u8 & 0x1) << 7);

    // flags
    emulator.cpu.registers.status.set(register::Status::C, carry);
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
    let offset = cpu::read_program_byte(emulator) as i8; // signed

    if should_branch {
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
    let signed_value = value as i8;
    let result: i8 = signed_value.wrapping_add(-1);

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result < 0);
    
    // result
    return result as u8;
}

fn inc(emulator: &mut config::Emulator, value: u8) -> u8 {
    let result: u8 = value.wrapping_add(1);

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

fn lda(emulator: &mut config::Emulator, value: u8) {
    // flags
    emulator.cpu.registers.status.set(register::Status::Z, value == 0);
    emulator.cpu.registers.status.set(register::Status::N, value & 0x80 == 0x80);
    
    // registers
    emulator.cpu.registers.a = value;
}

fn ldx(emulator: &mut config::Emulator, value: u8) {
    // flags
    emulator.cpu.registers.status.set(register::Status::Z, value == 0);
    emulator.cpu.registers.status.set(register::Status::N, value & 0x80 == 0x80);
    
    // registers
    emulator.cpu.registers.x = value;
}

fn ldy(emulator: &mut config::Emulator, value: u8) {
    // flags
    emulator.cpu.registers.status.set(register::Status::Z, value == 0);
    emulator.cpu.registers.status.set(register::Status::N, value & 0x80 == 0x80);
    
    // registers
    emulator.cpu.registers.y = value;
}


// UNOFICAL OPCODES
fn aac(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a & value;

    // flags
    emulator.cpu.registers.status.set(register::Status::C, result & 0x80 == 0x80);
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);
}

fn aax(emulator: &mut config::Emulator) -> u8 {
    let result = emulator.cpu.registers.a & emulator.cpu.registers.x;

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);

    return result;
}

fn arr(emulator: &mut config::Emulator, value: u8) {
    let result = emulator.cpu.registers.a & value;

    // flags
    emulator.cpu.registers.status.set(register::Status::Z, result == 0);
    emulator.cpu.registers.status.set(register::Status::N, result & 0x80 == 0x80);

    emulator.cpu.registers.a = emulator.cpu.registers.a >> 1;

    if emulator.cpu.registers.a & 0x20 == 0x20 && emulator.cpu.registers.a & 0x40 == 0x40 {
        emulator.cpu.registers.status.set(register::Status::C, true);
        emulator.cpu.registers.status.set(register::Status::V, false);
    } else if emulator.cpu.registers.a & 0x20 != 0x20 && emulator.cpu.registers.a & 0x40 != 0x40 {
        emulator.cpu.registers.status.set(register::Status::C, false);
        emulator.cpu.registers.status.set(register::Status::V, false);
    } else if emulator.cpu.registers.a & 0x20 == 0x20 && emulator.cpu.registers.a & 0x40 != 0x40 {
        emulator.cpu.registers.status.set(register::Status::C, false);
        emulator.cpu.registers.status.set(register::Status::V, true);
    } else if emulator.cpu.registers.a & 0x20 != 0x20 && emulator.cpu.registers.a & 0x40 == 0x40 {
        emulator.cpu.registers.status.set(register::Status::C, true);
        emulator.cpu.registers.status.set(register::Status::V, true);
    }
}