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

    keypad: [bool; 16],
    
    // Schermo 64x32 pixel (monocromatico)
    display: [bool; 64 * 32],
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
            keypad: [false; 16],
            display: [false; 64 * 32],
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

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // Qui andrebbe il BEEP!
            }
            self.sound_timer -= 1;
        }
    }

    // funzione provvisoria per aggiornare lo schermo (da implementare con una libreria grafica)
    pub fn print_display(&self) {
        print!("\x1B[H"); 

        for y in 0..32 {
            for x in 0..64 {
                let idx = x + (y * 64);
                if self.display[idx] {
                    print!("█"); 
                } else {
                    print!(" "); 
                }
            }
            println!();
        }
    }

    pub fn fetch(&mut self) -> u16 {
        // 1. leggi byte1 da memory[pc]
        let byte1 = self.memory[self.pc as usize] as u16;
        // 2. leggi byte2 da memory[pc + 1]
        let byte2 = self.memory[self.pc as usize + 1] as u16;
        // 3. combinali
        let opcode = (byte1 << 8) | byte2;
        // 4. incrementa pc di 2
        self.pc += 2;
        // 5. ritorna l'opcode
        opcode
    }

    pub fn execute(&mut self, opcode: u16) {
        // scomposizione dell'opcode (composto da 4 nibbles e 2 byte)
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;

        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        
        // match
        match c {
            0x0 => match nn {
                0xE0 => { 
                    self.display = [false; 64 * 32];
                    self.print_display(); // Aggiorna lo schermo dopo averlo pulito
                 },
                0xEE => { 
                    // RET: Torna indietro usando lo Stack
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                },
                _ => { /* Altri casi 0nnn raramente usati */ }
            },
            0x1 => { // jump
                self.pc = nnn;
            },
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            0x3 => {
                if(self.v[x] == nn){
                    self.pc += 2;
                }
            },
            0x4 => {
                if(self.v[x] != nn){
                    self.pc += 2;
                }
            },
            0x5 => {
                if(self.v[x] == self.v[y]){
                    self.pc += 2;
                }
            },
            0x6 => { // set a register vx
                self.v[x] = nn;
            },
            0x7 => { // add value to a register vx
                self.v[x] = self.v[x].wrapping_add(nn);
            },
            0x8 => match n {
                0x0 => self.v[x] = self.v[y],
                0x1 => self.v[x] |= self.v[y],
                0x2 => self.v[x] &= self.v[y],
                0x3 => self.v[x] ^= self.v[y],
                0x4 => {
                    let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = res;
                    self.v[0xF] = if overflow { 1 } else { 0 };
                },
                0x5 => {
                    if (self.v[x] > self.v[y]) {
                        self.v[0xF] = 1; // No borrow
                    } else {
                        self.v[0xF] = 0; // Borrow
                    }
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                },
                0x6 => {
                    if (self.v[x] & 0x1) == 1 {
                        self.v[0xF] = 1; // Least significant bit is 1
                    } else {
                        self.v[0xF] = 0; // Least significant bit is 0
                    }
                    self.v[x] >>= 1; // Divide Vx by 2 (shift right)
                },
                0x7 => {
                    if (self.v[y] > self.v[x]) {
                        self.v[0xF] = 1; // No borrow
                    } else {
                        self.v[0xF] = 0; // Borrow
                    }
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                },
                0xE => {
                    if (self.v[x] & 0x80) == 0x80 {
                        self.v[0xF] = 1; // Most significant bit is 1
                    } else {
                        self.v[0xF] = 0; // Most significant bit is 0
                    }
                    self.v[x] <<= 1; // Multiply Vx by 2 (shift left)
                },
                _ => println!("Opcode 8 sconosciuto: {:X}", n),
            },
            0x9 => {
                if (self.v[x] != self.v[y]) {
                    self.pc += 2;
                }
            },
            0xA => {
                self.i = nnn;
            },
            0xB => {
                self.pc = nnn + self.v[0] as u16;
            },
            0xC => {
                let random_byte: u8 = rand::random();
                self.v[x] = random_byte & nn;
            },
            0xD => {
                let x_coord = self.v[x] as usize % 64;
                let y_coord = self.v[y] as usize % 32;
                let height = n as usize;
                
                self.v[0xF] = 0; // Reset collision flag

                for row in 0..height {
                    let sprite_byte = self.memory[(self.i + row as u16) as usize];
                    
                    for col in 0..8 {
                        // Controlla se il bit specifico (dal più significativo al meno) è 1
                        if (sprite_byte & (0x80 >> col)) != 0 {
                            let px = (x_coord + col) % 64;
                            let py = (y_coord + row) % 32;
                            
                            // Indice nell'array monodimensionale del display
                            let idx = px + (py * 64);
                            
                            if self.display[idx] {
                                self.v[0xF] = 1; // Collisione rilevata
                            }
                            
                            self.display[idx] ^= true; // Disegna con XOR
                        }
                    }
                }
                self.print_display(); // Aggiorna lo schermo dopo aver disegnato
            }

            _ => println!("Opcode non gestito: {:#X}", opcode),
        }
    }
}
