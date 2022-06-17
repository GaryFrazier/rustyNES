use crate::config;

/* instructions handled by the cpu, structured as a tuple in the format
(name, opcode, number of bytes, execution function that returns the number of cycles it took to complete)

the name field will indicate the addressing mode with the following codes
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
        emulator.cpu.registers.a = 1;
        return 2
    })
];
