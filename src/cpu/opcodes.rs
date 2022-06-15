/* instructions handled by the cpu, structured as a tuple in the format
(name, opcode, number of bytes, execution function)

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
pub static Opcodes = [
    ("ADC- I",  0x69,  2, |x: i32, y| if x < y { x } else { y }; ) //todo func is an example, relace it
];