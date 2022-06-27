mod config;
mod cpu;
mod ram;
mod rom;
mod ppu;
use std::env;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

    init_canvas(&mut emulator).expect("initialization failed");
}

fn init_canvas(emulator: &mut config::Emulator) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rustyNES", 256, 240)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    //let mut cycle = 0;
    'running: loop {
        //cycle = (cycle + 1) % ppu::CYCLES_PER_SCANLINE;
        //canvas.set_draw_color(Color::RGB(cycle, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        for _ in 1..29781 {
            ppu::run_cycle(emulator);
            ppu::run_cycle(emulator);
            ppu::run_cycle(emulator);

            cpu::run_cycle(emulator);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}