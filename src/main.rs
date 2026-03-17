mod cpu;
mod platform;

use cpu::Cpu;
use platform::Platform;
use std::thread;
use std::time::Duration;

fn main() {
    let scale = 15;
    let (mut platform, mut event_pump) = Platform::new("Rust Chip-8", 64, 32, scale);

    let mut my_cpu = Cpu::new();
    my_cpu.load_rom(r"chip8-roms-master\tests\IBM Logo.ch8");

    'gameloop: loop {
        // Eventi tastiera
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'gameloop,
                // mettere mapping tasti
                _ => {}
            }
        }

        // Operazioni CPU
        for _ in 0..10 {
            let opcode = my_cpu.fetch();
            my_cpu.execute(opcode);
        }
        my_cpu.tick_timers();

        // Aggiornamento schermo
        platform.draw(&my_cpu.display);

        // pausa
        thread::sleep(Duration::from_millis(16));
    }
}
