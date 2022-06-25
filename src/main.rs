mod config;
mod cpu;
mod ram;
mod rom;
mod ppu;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file_name = (&args[1]).to_string();
    boot(rom_file_name);
}

fn boot(file_name: String) {
    let mut emulator = config::Emulator::default();
    match emulator.rom.load_file(&file_name) {
        Ok(()) => println!("{} loaded", file_name),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    rom::init_mapper(&mut emulator);
    cpu::reset(&mut emulator);
    ppu::reset(&mut emulator);
    println!("{}", emulator);

    //let mut cycle = 0;
    // main loop
    while !emulator.shut_down {
        cpu::run_cycle(&mut emulator);
        ppu::run_cycle(&mut emulator);
        //cycle += 1;
    }
}