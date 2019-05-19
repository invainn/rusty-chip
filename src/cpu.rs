use std::fs::File;
use std::io::prelude::*;

use crate::display::{ FONT_SET, CHIP8_HEIGHT, CHIP8_WIDTH };

use rand::Rng;

pub struct CPU {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: usize,
    pub pc: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    pub keypad: [bool; 16],
    pub key_wait: bool,
    pub key_wait_reg: usize,
    pub gfx: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub draw_flag: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0u16,
            memory: [0u8; 4096],
            v: [0u8; 16],
            i: 0usize,
            pc: 0usize,
            delay_timer: 0u8,
            sound_timer: 0u8,
            stack: [0u16; 16],
            sp: 0usize,
            keypad: [false; 16],
            key_wait: false,
            key_wait_reg: 0usize,
            gfx: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            draw_flag: false,
        }
    }

    pub fn initialize(&mut self, game: String) {
        self.pc = 0x200;
        self.opcode = 0;
        self.i = 0;
        self.sp = 0;

        // Clear display, stack, registers, and memory
        self.stack = [0u16; 16];
        self.memory = [0u8; 4096];

        for i in 0..79 {
            self.memory[i] = FONT_SET[i];
        }

        let mut game_file = File::open(game).expect("Game not found");
        let mut buffer: Vec<u8> = Vec::new();

        // Read the game file into a buffer
        game_file.read_to_end(&mut buffer).expect("Error reading game");

        // Populate game into memory
        for (i, &byte) in buffer.iter().enumerate() {
            let addr_pos = 0x200 + i;

            if addr_pos < 4096 {
                self.memory[addr_pos] = byte;
            } else {
                break;
            }
        }

        // Reset timers
        self.delay_timer = 0u8;
        self.sound_timer = 0u8;
    }

    // emulate_cycle
    // fetches, decodes, and executes the opcycle
    // updates the timers as well
    pub fn emulate_cycle(&mut self, keypad: [bool; 16]) {
        // Store the keypad for opcodes to access it
        self.keypad = keypad;

        // If we're waiting for a keypress, then skip opcode execution
        // and register the keypress
        if self.key_wait {
            for (i, &key) in keypad.iter().enumerate() {
                if key {
                    self.key_wait = false;
                    self.v[self.key_wait_reg] = i as u8;
                }
            }
        } else {
            // FETCH
            // Extract the opcode and store in extracted_op
            // We read in one byte, then shift left 8 bits
            // then read the next byte and bitwise-OR it to grab the full word
            // and store it
            let extracted_op: u16 = (self.memory[self.pc] as u16) << 8 
                                | self.memory[self.pc + 1] as u16;

            // Store that in CPU's opcode
            self.opcode = extracted_op;

            // Decode will be done in an other method
            // then decode will call another method to execute the opcode
            self.decode_opcode();
        }

    }

    // TODO: Legacy mode?
    // Opcodes 8XY6, 8XYE, FX55, and FX65 are debated to have different functionality
    // Newer roms work with only one spec, while older games work with the other
    //
    // decodes the opcode and matches based on the first nibble
    // then executes the correct opcode function
    pub fn decode_opcode(&mut self) {
        println!("{:X}", self.opcode);
        match (self.opcode & 0xF000) >> 12 {
            0x0 => self.decode_0(),
            0x1 => self.oc_1nnn(),
            0x2 => self.oc_2nnn(),
            0x3 => self.oc_3xkk(),
            0x4 => self.oc_4xkk(),
            0x5 => self.oc_5xy0(),
            0x6 => self.oc_6xkk(),
            0x7 => self.oc_7xkk(),
            0x8 => self.decode_8(),
            0x9 => self.oc_9xy0(),
            0xA => self.oc_annn(),
            0xB => self.oc_bnnn(),
            0xC => self.oc_cxkk(),
            0xD => self.oc_dxyn(),
            0xE => self.decode_e(),
            0xF => self.decode_f(),
            _   => panic!("not implemented {}", self.opcode),
        }
    }
    
    fn decode_0(&mut self) {
        match self.opcode {
            0x00E0 => self.oc_00e0(),
            0x00EE => self.oc_00ee(),
            _      => self.oc_0nnn(),
        }
    }

    fn decode_8(&mut self) {
        match self.opcode & 0x000F {
            0x0 => self.oc_8xy0(),
            0x1 => self.oc_8xy1(),
            0x2 => self.oc_8xy2(),
            0x3 => self.oc_8xy3(),
            0x4 => self.oc_8xy4(),
            0x5 => self.oc_8xy5(),
            0x6 => self.oc_8xy6(),
            0x7 => self.oc_8xy7(),
            0xE => self.oc_8xye(),
            _   => panic!("not implemented {}", self.opcode),
        }
    }

    fn decode_f(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => self.oc_fx07(),
            0x0A => self.oc_fx0a(),
            0x15 => self.oc_fx15(),
            0x18 => self.oc_fx18(),
            0x1E => self.oc_fx1e(),
            0x29 => self.oc_fx29(),
            0x33 => self.oc_fx33(),
            0x55 => self.oc_fx55(),
            0x65 => self.oc_fx65(),
            _    => panic!("not implemented {}", self.opcode),
        }
    }

    fn decode_e(&mut self) {
        match self.opcode & 0x00FF {
            0xA1 => self.oc_exa1(),
            0x9E => self.oc_ex9e(),
            _   => panic!("not implemented {}", self.opcode),
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // Jump to machine code routine
    // Ignored
    fn oc_0nnn(&mut self) {
        self.pc += 2;
    }

    // Clear display
    fn oc_00e0(&mut self) {
        self.gfx = [[0; CHIP8_WIDTH]; CHIP8_HEIGHT];
        self.draw_flag = true;
        self.pc += 2;
    }

    // Return from subroutine
    fn oc_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize] as usize;
    }

    // JMP to nnn
    fn oc_1nnn(&mut self) {
        let addr = (self.opcode & 0x0FFF) as usize;
        self.pc = addr; 
    }

    // Call subroutine
    // Increments stack pointer and sets current PC at top of stack
    // then set PC to nnn
    fn oc_2nnn(&mut self) {
        let addr = (self.opcode & 0x0FFF) as usize;

        self.stack[self.sp as usize] = self.pc as u16 + 2;
        self.sp += 1;

        self.pc = addr;
    }

    // Skip next instruction if Vx = kk
    fn oc_3xkk(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = (self.opcode & 0x00FF) as u8; 

        if self.v[x] == kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Skip next instruction if Vx != kk
    fn oc_4xkk(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = (self.opcode & 0x00FF) as u8; 

        if self.v[x] != kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Skip next instruction if Vx = Vy
    fn oc_5xy0(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[x] == self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Vx = kk
    fn oc_6xkk(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = (self.opcode & 0x00FF) as u8; 

        self.v[x] = kk;

        self.pc += 2;
    }

    // Vx = Vx + kk
    fn oc_7xkk(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = (self.opcode & 0x00FF) as u8; 

        self.v[x] = self.v[x].wrapping_add(kk);

        self.pc += 2;
    }

    // Vx = Vy 
    fn oc_8xy0(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v[x] = self.v[y];

        self.pc += 2;
    }

    // Vx = Vx OR Vy
    fn oc_8xy1(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v[x] |= self.v[y];

        self.pc += 2;
    }

    // Vx = Vx AND Vy
    fn oc_8xy2(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v[x] &= self.v[y];

        self.pc += 2;
    }
    // Vx = Vx XOR Vy
    fn oc_8xy3(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v[x] ^= self.v[y];

        self.pc += 2;
    }

    // Vx = Vx + Vy
    fn oc_8xy4(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        let sum = self.v[x] as u16 + self.v[y] as u16;

        if sum > 255 {
            self.v[15] = 1;
        } else {
            self.v[15] = 0
        }

        self.v[x] = (sum & 0x00FF) as u8;

        self.pc += 2;
    }

    // Vx = Vx - Vy
    fn oc_8xy5(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[y] > self.v[x] {
            self.v[15] = 0;
        } else {
            self.v[15] = 1;
        }

        self.v[x] = self.v[x].wrapping_sub(self.v[y]);

        self.pc += 2;
    }

    // If LSB = 1, then Vf = 1
    fn oc_8xy6(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        self.v[15] = self.v[x] & 1;
        self.v[x] = self.v[y] >> 1;

        self.pc += 2;
    }

    // Vx = Vy - Vx 
    // Set Vf to not borrow
    fn oc_8xy7(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[x] > self.v[y] {
            self.v[15] = 0;
        } else {
            self.v[15] = 1; 
        }

        self.v[x] = self.v[y].wrapping_sub(self.v[x]);

        self.pc += 2;
    }

    fn oc_8xye(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        // Set FLAG to MSB
        self.v[15] = self.v[x] >> 7;
        self.v[x] <<= 1;

        self.pc += 2;
    }

    // Skip next ins if Vx != Vy 
    fn oc_9xy0(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[x] != self.v[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Set register I to nnn
    fn oc_annn(&mut self) {
        let addr = (self.opcode & 0xFFF) as usize;
        self.i = addr;

        self.pc += 2;
    }

    // JMP to nnn + V0
    fn oc_bnnn(&mut self) {
        let addr = (self.opcode & 0xFFF) as usize;

        self.pc = addr + self.v[0] as usize;
    }

    // Generate number from 0-255 and bitwise-AND with kk
    // store value in Vx
    fn oc_cxkk(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let kk = (self.opcode & 0x00FF) as u8; 

        let mut rng = rand::thread_rng();

        self.v[x] = rng.gen_range(0, 255) & kk;

        self.pc += 2;
    }

    // Draw sprite onto the screen at coords (Vx, Vy)
    // If Vf is set, then there is a collision
    // Sprite is located at location I 
    fn oc_dxyn(&mut self) {
        let x = self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize;
        let y = self.v[((self.opcode & 0x00F0) >> 4) as usize] as usize;
        let height = (self.opcode & 0x000F) as usize;

        // Reset Vf flag
        self.v[15] = 0;

        // Logic mostly taken from multigesture
        // Modified to work with a 2D array
        for row in 0..height {
            let pixel = self.memory[self.i + row];

            for col in 0..8 {
                // Check if the current pixel is set 
                if (pixel & (0x80 >> col)) != 0 {
                    // Check if current display pixel is set to 1
                    if self.gfx[(y + row) % CHIP8_HEIGHT][(x + col) % CHIP8_WIDTH] == 1 {
                        // Set Vf for any pixel that is set from 1 to 0
                        self.v[15] |= 1;
                    }
                    // XOR the bit
                    self.gfx[(y + row) % CHIP8_HEIGHT][(x + col) % CHIP8_WIDTH] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        self.pc += 2;
    }

    // If the keypad with value Vx if pressed,
    // then skip next instruction
    fn oc_ex9e(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        if self.keypad[self.v[x] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // If keypad with value Vx is not pressed,
    // skip next instruction
    fn oc_exa1(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        if self.keypad[self.v[x] as usize] == false {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // Vx = delay timer value
    fn oc_fx07(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.v[x] = self.delay_timer;

        self.pc += 2;
    }

    // Wait for keypress
    fn oc_fx0a(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.key_wait = true;
        self.key_wait_reg = x; 

        self.pc += 2;
    }

    // Set delay timer to Vx
    fn oc_fx15(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        
        self.delay_timer = self.v[x];

        self.pc += 2;
    }

    fn oc_fx18(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.sound_timer = self.v[x];

        self.pc += 2;
    }

    // I = I + Vx
    fn oc_fx1e(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.i += self.v[x] as usize;

        self.pc += 2;
    }

    // Set I to location of hex sprite (fonts)
    fn oc_fx29(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.i = self.v[x] as usize * 5;

        self.pc += 2;
    }

    // Store BCD representation of Vx in I, I+1, and I+2
    fn oc_fx33(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] / 10) % 10;
        self.memory[self.i + 2] = (self.v[x] % 100) % 10;

        self.pc += 2;
    }

    // Store registers V0 to Vx starting from memory[I] 
    fn oc_fx55(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        for ind in 0..=x {
            self.memory[self.i + ind] = self.v[ind];
        }

        self.pc += 2;
    }

    // Read registers V0 to Vx from memory[I]
    fn oc_fx65(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;

        for ind in 0..=x {
            self.v[ind] = self.memory[self.i + ind];
        }

        self.pc += 2;
    }
}

