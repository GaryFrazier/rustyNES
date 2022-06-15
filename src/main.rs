mod config;
mod cpu;

fn main() {
    let emulator = config::Emulator::default();
    println!("{}", emulator)
}
