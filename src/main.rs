mod cpu;
fn main() {
    let mut my_cpu = cpu::Cpu::new();

    my_cpu.load_rom(r"chip8-roms-master\games\Connect 4 [David Winter].ch8");

    println!("{:?}", my_cpu);
}
