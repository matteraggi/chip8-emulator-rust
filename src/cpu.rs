use std::fs;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Cpu {
    // 4096 bytes of RAM
    memory: [u8; 4096],

    // 16 general-purpose 8-bit registers (V0 to VF)
    v: [u8; 16],

    // Index register (16-bit, but usually only 12 bits are used)
    i: u16,

    // Program Counter (16-bit)
    pc: u16,

    // Stack Pointer (8-bit is enough, as Chip-8 stack has 16 levels)
    sp: u8,

    // The Stack (16 levels of 16-bit addresses)
    stack: [u16; 16],

    // Delay Timer (8-bit, decrements at 60Hz)
    delay_timer: u8,

    // Sound Timer (8-bit, decrements at 60Hz)
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200, // Il programma inizia all'indirizzo 0x200, i primi 512 byte sono riservati per il sistema
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        };
        cpu.load_fonts();
        cpu
    }

    pub fn load_fonts(&mut self) {
        for i in 0..FONTSET.len() {
            self.memory[i] = FONTSET[i];
        }
        println!("Fontset caricato in memoria!");
    }

    pub fn load_rom(&mut self, path: &str) {
        let data = fs::read(path).expect("Errore: Impossibile leggere la ROM");

        if data.len() > (4096 - 0x200) {
            panic!("Errore: La ROM è troppo grande per la memoria del Chip-8");
        }

        for (i, &byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }

        println!("ROM caricata con successo: {} byte", data.len());
    }
}
