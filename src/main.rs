mod config;
mod cpu;
mod ram;

fn main() {
    let mut emulator = config::Emulator::default();
    cpu::run_next_instruction(&mut emulator);
    println!("{}", emulator)
}
