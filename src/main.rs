use std::thread;
use std::time::Duration;
mod cpu;
fn main() {
    let mut my_cpu = cpu::Cpu::new();

    my_cpu.load_rom(r"chip8-roms-master\tests\IBM Logo.ch8");

    println!("{:?}", my_cpu);
    
    loop {
        for _ in 0..10 {
            let opcode = my_cpu.fetch();
            my_cpu.execute(opcode);
        }

        my_cpu.tick_timers();

        my_cpu.print_display();

        // Pausa per stabilizzare a circa 60 cicli al secondo
        thread::sleep(Duration::from_millis(16));
    }
}
