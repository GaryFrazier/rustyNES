use crate::config;
use crate::cpu;
use crate::cpu::register;

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
pub static OPCODES: [(&str, u8, i32, fn(&mut config::Emulator) -> u32); 1] = [
    ("ADC- I",  0x69,  2, |emulator: &mut config::Emulator| -> u32 {
        let add_value: u16 = cpu::read_program_byte(emulator) as u16;
        let total: u16 = emulator.cpu.registers.a as u16 
            + add_value
            + emulator.cpu.registers.status.contains(register::Status::C) as u16;

        // flags
        emulator.cpu.registers.status.set(register::Status::C, total > 0xFF);
        emulator.cpu.registers.status.set(register::Status::Z, total & 0xFF == 0);
        emulator.cpu.registers.status.set(register::Status::V, (emulator.cpu.registers.a as u16 ^ total) & (add_value ^ total) & 0x80 == 0x80); // if pos + pos = neg or neg + neg = pos, explanations here https://forums.nesdev.org/viewtopic.php?t=6331
        emulator.cpu.registers.status.set(register::Status::N, total & 0x80 == 0x80);
        
        // registers
        emulator.cpu.registers.a = (total & 0xFF) as u8;
        
        return 2
    })
];
