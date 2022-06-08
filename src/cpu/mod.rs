mod register;

/* 
    The NES cpu is a modified version of the 6502 processing unit, instructions
    defined here will be reflecting those of the 6502.
*/
pub fn initialize() {
    let registers: register::Registers = register::Registers{status: ..Default::default()};
    println!("{}", format!("{:?}", registers.status))
}

